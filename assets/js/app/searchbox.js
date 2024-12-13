export function setupDropdown() {
  const dropDownListingTypeEl = document.getElementById(
    "listing-type-dropdown"
  );
  const dropDownLocationEl = document.getElementById("location-dropdown");
  const dropDownPropertyEl = document.getElementById("property-types-dropdown");

  addEventListenerToDropdown(dropDownListingTypeEl, "listingType");
  addEventListenerToDropdown(dropDownLocationEl, "location");
  addEventListenerToDropdown(dropDownPropertyEl, "propertyType");
}

function addEventListenerToDropdown(dropDownContainerEl, type) {
  const dropDownEl =
    dropDownContainerEl.parentElement.querySelector(".dropdown");

  dropDownContainerEl.addEventListener("click", () => {
    let childNode =
      type === "listingType"
        ? dropDownEl.childNodes
        : dropDownEl.childNodes[0].childNodes;

    let height = 1.93 * childNode.length;

    if (type !== "listingType") {
      height = 2.1 * childNode.length;
    }

    if (childNode.length < 6 && type !== "listingType") {
      dropDownEl.classList.remove("overflow-scroll");
      dropDownEl.classList.add("overflow-hidden");
    }

    if (dropDownEl.classList.contains("invisible")) {
      dropDownEl.classList.remove(
        "invisible",
        "pointer-events-none",
        "opacity-0",
        "max-h-0"
      );

      dropDownEl.classList.add("opacity-100", "max-h-50");
      dropDownEl.style.height = `${height}rem`;
    } else {
      dropDownEl.style.height = 0;
      dropDownEl.classList.remove("opacity-100", "max-h-50");
      dropDownEl.classList.add(
        "invisible",
        "pointer-events-none",
        "opacity-0",
        "max-h-0"
      );
    }
  });
}
