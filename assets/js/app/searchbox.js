export function setupDropdown() {
  const dropDownTitleEl = document.getElementById(
    "listing-type-dropdown-title"
  );
  const dropDownEl = document.getElementById("listing-type-dropdown");

  dropDownTitleEl.addEventListener("click", () => {
    if (dropDownEl.classList.contains("invisible")) {
      dropDownEl.classList.remove(
        "invisible",
        "pointer-events-none",
        "h-0",
        "opacity-0"
      );
      dropDownEl.classList.add("h-31", "opacity-100");
    } else {
      dropDownEl.classList.remove("h-31", "opacity-100");
      dropDownEl.classList.add(
        "invisible",
        "pointer-events-none",
        "h-0",
        "opacity-0"
      );
    }
  });
}
