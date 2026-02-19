import React, { useState, useEffect } from "react";
import {
  APIProvider,
  Map,
  AdvancedMarker,
  InfoWindow,
} from "@vis.gl/react-google-maps";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";

// Read the API key from the environment variable
const GOOGLE_MAPS_API_KEY = "[REDACTED_FRONTEND_KEY]";

function GpsMap() {
  const [coordinates, setCoordinates] = useState([]);
  const [filteredCoordinates, setFilteredCoordinates] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState(null);
  const [startDate, setStartDate] = useState(new Date());
  const [endDate, setEndDate] = useState(new Date());

  useEffect(() => {
    fetchCoordinates();
  }, []);

  // Add this useEffect to inject custom CSS for date picker styling
  useEffect(() => {
    const style = document.createElement("style");
    style.textContent = `
      .date-picker-input {
        padding: 8px 12px;
        border: 1px solid #ccc;
        border-radius: 4px;
        font-size: 13px;
        min-width: 80px;
		width: 105px;
        transition: border-color 0.15s ease-in-out, box-shadow 0.1s ease-in-out;
      }

      .date-picker-input:focus {
        outline: none;
        border-color: #0056d2f;
        box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
      }

      .react-datepicker {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
        border: 1px solid #dee2e6;
        border-radius: 6px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      }

      .react-datepicker__header {
        background-color: #f8f9fa;
        border-bottom: 1px solid #dee2e6;
        border-radius: 6px 6px 0 0;
      }

      .react-datepicker__day--selected {
        background-color: #0056d2f;
        color: white;
        border-radius: 4px;
      }

      .react-datepicker__day--keyboard-selected {
        background-color: #e9ecef;
        border-radius: 4px;
      }

      .react-datepicker__day:hover {
        background-color: #f8f9fa;
        border-radius: 4px;
      }

      .react-datepicker__navigation {
        top: 12px;
      }

      .react-datepicker__navigation--previous {
        left: 12px;
      }

      .react-datepicker__navigation--next {
        right: 12px;
      }
    `;
    document.head.appendChild(style);

    // Cleanup function to remove the style when component unmounts
    return () => {
      document.head.removeChild(style);
    };
  }, []);

  // Filter coordinates when date range changes
  useEffect(() => {
    filterCoordinatesByDateRange();
  }, [startDate, endDate, coordinates]);

  const fetchCoordinates = async () => {
    try {
      const response = await fetch("https://redacted-api.invalid/api/coordinates");
      const data = await response.json();
      setCoordinates(data.coordinates);
      setLoading(false);
    } catch (error) {
      console.error("Error fetching coordinates:", error);
      setLoading(false);
    }
  };

  // Helper function to parse the Rust date format: "2024-01-15 · 02:30 PM"
  const parseRustDate = (dateString) => {
    if (!dateString) return null;

    // Extract just the date part (before the ' · ')
    const datePart = dateString.split(" · ")[0];

    // Split the date string and create a local date (no timezone shift)
    const [year, month, day] = datePart.split("-").map(Number);
    return new Date(year, month - 1, day); // month is 0-indexed in JavaScript
  };

  // Helper function to compare dates (date only, ignoring time)
  const isSameDate = (date1, date2) => {
    if (!date1 || !date2) return false;
    return date1.toDateString() === date2.toDateString();
  };

  const isDateInRange = (
    eventStartDate,
    eventEndDate,
    filterStart,
    filterEnd,
  ) => {
    if (!eventStartDate) return false;
    if (!filterStart && !filterEnd) return true;

    // Create date-only versions for comparison
    const eventStart = new Date(
      eventStartDate.getFullYear(),
      eventStartDate.getMonth(),
      eventStartDate.getDate(),
    );
    const eventEnd = eventEndDate
      ? new Date(
          eventEndDate.getFullYear(),
          eventEndDate.getMonth(),
          eventEndDate.getDate(),
        )
      : eventStart;

    const filterStartOnly = filterStart
      ? new Date(
          filterStart.getFullYear(),
          filterStart.getMonth(),
          filterStart.getDate(),
        )
      : null;
    const filterEndOnly = filterEnd
      ? new Date(
          filterEnd.getFullYear(),
          filterEnd.getMonth(),
          filterEnd.getDate(),
        )
      : null;

    // Check for overlap between event range and filter range
    if (filterStartOnly && filterEndOnly) {
      // Both filter dates provided - check for overlap
      return eventStart <= filterEndOnly && eventEnd >= filterStartOnly;
    } else if (filterStartOnly) {
      // Only start filter date - event must end on or after filter start
      return eventEnd >= filterStartOnly;
    } else if (filterEndOnly) {
      // Only end filter date - event must start on or before filter end
      return eventStart <= filterEndOnly;
    }

    return true;
  };

  const filterCoordinatesByDateRange = () => {
    if (!startDate && !endDate) {
      setFilteredCoordinates([...coordinates]);
      return;
    }

    const filtered = coordinates.filter((coord) => {
      const eventStartDate = parseRustDate(coord.start_time);
      const eventEndDate = parseRustDate(coord.end_time);
      return isDateInRange(eventStartDate, eventEndDate, startDate, endDate);
    });

    setFilteredCoordinates(filtered);
  };

  // Quick date filter functions
  const setToday = () => {
    const today = new Date();
    setStartDate(today);
    setEndDate(today);
  };

  const setThisWeek = () => {
    const today = new Date();
    const currentDay = today.getDay(); // 0 = Sunday, 1 = Monday, etc.

    let startDate, endDate;

    // If today is Saturday (6) or Sunday (0), show next week
    if (currentDay === 0 || currentDay === 6) {
      // Calculate next Monday
      const nextMonday = new Date(today);
      const daysUntilMonday = currentDay === 0 ? 1 : 2; // Sunday: 1 day, Saturday: 2 days
      nextMonday.setDate(today.getDate() + daysUntilMonday);

      // Calculate next Friday
      const nextFriday = new Date(nextMonday);
      nextFriday.setDate(nextMonday.getDate() + 4);

      startDate = nextMonday;
      endDate = nextFriday;
    }
    // If today is Monday-Friday, show from today through Friday
    else {
      // Start from today
      startDate = new Date(today);

      // Calculate Friday of current week
      const friday = new Date(today);
      const daysUntilFriday = 5 - currentDay; // Days remaining until Friday
      friday.setDate(today.getDate() + daysUntilFriday);

      endDate = friday;
    }

    setStartDate(startDate);
    setEndDate(endDate);
  };

  const setThisWeekend = () => {
    const today = new Date();
    const currentDay = today.getDay(); // 0 = Sunday, 1 = Monday, etc.

    // Calculate Saturday
    const saturday = new Date(today);
    const daysUntilSaturday = currentDay === 0 ? -1 : 6 - currentDay; // If Sunday, go back 1 day
    saturday.setDate(today.getDate() + daysUntilSaturday);

    // Calculate Sunday
    const sunday = new Date(saturday);
    sunday.setDate(saturday.getDate() + 1);

    setStartDate(saturday);
    setEndDate(sunday);
  };

  const clearDateFilter = () => {
    setStartDate(null);
    setEndDate(null);
  };

  if (loading) {
    return <div>Loading map and data from API...</div>;
  }

  const center = { lat: 47.6061, lng: -122.3328 }; // Default to Seattle

  // Existing InfoWindow styles
  const infoWindowStyles = {
    content: {
      position: "relative",
      padding: "15px",
      paddingTop: "20px",
      fontFamily:
        '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
      color: "#333",
      maxWidth: "180px",
    },
    title: {
      fontSize: "0.85rem",
      fontWeight: "bold",
      margin: "0 0 5px 0",
      color: "#1a1a1a",
    },
    attendance: {
      fontSize: "0.85rem",
      fontStyle: "italic",
      color: "#555",
      margin: "0 0 10px 0",
    },
    details: {
      margin: "3.7px 0",
      fontSize: "0.8rem",
    },
    url: {
      margin: "3px 0",
      fontSize: "0.8rem",
    },
    closeButton: {
      position: "absolute",
      top: "3px",
      right: "4px",
      background: "transparent",
      border: "none",
      padding: "0",
      cursor: "pointer",
      fontSize: "17px",
      color: "#333",
      lineHeight: "1",
      outline: "none",
    },
  };

  const dateFilterContainerStyle = {
    position: "absolute",
    top: "1px",
    left: "35px",
    right: "35px",
    zIndex: 1000,
    backgroundColor: "white",
    borderRadius: "8px",
    boxShadow: "0 2px 10px rgba(0,0,0,0.1)",
    padding: "3px",
    display: "flex",
    flexDirection: "column",
    gap: "10px",
  };

  const datePickerRowStyle = {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    gap: "9px",
    flexWrap: "wrap",
  };

  const quickFilterRowStyle = {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    gap: "10px",
    flexWrap: "wrap",
  };

  const filterInfoStyle = {
    fontSize: "14px",
    color: "#666",
    display: "flex",
    alignItems: "center",
    gap: "10px",
  };

  const buttonStyle = {
    background: "#f0f0f0",
    border: "1px solid #ccc",
    borderRadius: "4px",
    padding: "6px 8px",
    fontSize: "12px",
    cursor: "pointer",
    transition: "background-color 0.15s ease-in-out",
  };

  const quickFilterButtonStyle = {
    background: "#0056d2f",
    color: "white",
    border: "1px solid #0056d2f",
    borderRadius: "4px",
    padding: "7px 9px",
    fontSize: "11.5px",
    cursor: "pointer",
    transition: "background-color 0.15s ease-in-out",
  };

  return (
    <APIProvider apiKey={GOOGLE_MAPS_API_KEY}>
      <div
        style={{
          height: "100vh",
          width: "100vw", // Use viewport width instead of 100%
          position: "fixed", // Fixed positioning for true full screen
          top: 0, // Remove any default top spacing
          left: 0, // Remove any default left spacing
          margin: 0, // Remove any default margins
          padding: 0, // Remove any default padding
        }}
      >
        {/* Date Range Filter */}
        <div style={dateFilterContainerStyle}>
          {/* Quick Filter Buttons */}
          <div style={quickFilterRowStyle}>
            {/*<label style={{ fontSize: '14px', fontWeight: '500' }}>Quick Filters:</label>*/}

            <button
              onClick={setToday}
              style={quickFilterButtonStyle}
              onMouseOver={(e) => (e.target.style.backgroundColor = "#0056b3")}
              onMouseOut={(e) => (e.target.style.backgroundColor = "#0056d2f")}
            >
              Today
            </button>

            <button
              onClick={setThisWeek}
              style={quickFilterButtonStyle}
              onMouseOver={(e) => (e.target.style.backgroundColor = "#0056b3")}
              onMouseOut={(e) => (e.target.style.backgroundColor = "#0056d2f")}
            >
              This Week
            </button>

            <button
              onClick={setThisWeekend}
              style={quickFilterButtonStyle}
              onMouseOver={(e) => (e.target.style.backgroundColor = "#0056b3")}
              onMouseOut={(e) => (e.target.style.backgroundColor = "#0056d2f")}
            >
              This Weekend
            </button>
          </div>

          {/* Custom Date Range */}
          <div style={datePickerRowStyle}>
            {/* <label style={{ fontSize: '14px', fontWeight: '500' }}>Custom Date Range:</label> */}

            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <DatePicker
                selected={startDate}
                onChange={(date) => setStartDate(date)}
                selectsStart
                startDate={startDate}
                endDate={endDate}
                placeholderText="Start date"
                dateFormat="MM/dd/yy"
                isClearable
                showYearDropdown
                showMonthDropdown
                dropdownMode="select"
                className="date-picker-input"
              />
            </div>

            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <DatePicker
                selected={endDate}
                onChange={(date) => setEndDate(date)}
                selectsEnd
                startDate={startDate}
                endDate={endDate}
                minDate={startDate}
                placeholderText="End date"
                dateFormat="MM/dd/yy"
                isClearable
                showYearDropdown
                showMonthDropdown
                dropdownMode="select"
                className="date-picker-input"
              />
            </div>

            <button
              onClick={clearDateFilter}
              style={buttonStyle}
              onMouseOver={(e) => (e.target.style.backgroundColor = "#e0e0e0")}
              onMouseOut={(e) => (e.target.style.backgroundColor = "#f0f0f0")}
            >
              Clear
            </button>
          </div>
          {/* 
          <div style={filterInfoStyle}>
            <span>
              Showing {filteredCoordinates.length} of {coordinates.length} locations
            </span>
          </div>
	  */}
        </div>

        <Map
          defaultZoom={11.3}
          defaultCenter={center}
          mapId={"[REDACTED_MAP_ID]"}
          style={{ paddingTop: "83px" }} // Increased padding for additional filter row
          mapTypeId="roadmap"
        >
          {filteredCoordinates.map((coord, index) => (
            <AdvancedMarker
              key={`${coord.label}-${coord.latitude}-${coord.longitude}-${index}`}
              position={{ lat: coord.latitude, lng: coord.longitude }}
              title={coord.label || "Location"}
              onClick={() => setSelected(coord)}
            >
              <img
                src="https://redacted.invalid/assets/map-pin.png"
                width="10"
                height="20"
                style={{ cursor: "pointer" }}
                alt="Location pin"
              />
            </AdvancedMarker>
          ))}

          {selected && (
            <InfoWindow
              position={{ lat: selected.latitude, lng: selected.longitude }}
              headerDisabled={true}
              onClose={() => setSelected(null)}
            >
              <div style={infoWindowStyles.content}>
                <button
                  style={infoWindowStyles.closeButton}
                  onClick={() => setSelected(null)}
                >
                  &times;
                </button>

                <h3 style={infoWindowStyles.title}>
                  {selected.label || "Location Details"}
                </h3>

                {selected.attendance && (
                  <p style={infoWindowStyles.attendance}>
                    <b>👥 </b> {selected.attendance}
                  </p>
                )}
                {selected.start_time && (
                  <p style={infoWindowStyles.details}>
                    <b>▶️ </b>
                    {selected.start_time}
                  </p>
                )}
                {selected.end_time && (
                  <p style={infoWindowStyles.details}>
                    <b>🔚</b>
                    {selected.end_time}
                  </p>
                )}
                {selected.website && (
                  <p style={infoWindowStyles.url}>
                    <b>🔗</b>
                    <a
                      href={selected.website}
                      target="_blank"
                      rel="noopener noreferrer"
                      style={{ marginLeft: "5px" }}
                    >
                      Link
                    </a>
                  </p>
                )}
              </div>
            </InfoWindow>
          )}
        </Map>
      </div>
    </APIProvider>
  );
}

export default GpsMap;
