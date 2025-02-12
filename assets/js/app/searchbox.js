export function setupDropdown() {
  const dropDownListingTypeEl = document.getElementById(
    "listing-type-dropdown"
  );
  const dropDownLocationEl = document.getElementById("location-dropdown");
  const dropDownPropertyEl = document.getElementById("property-types-dropdown");
  const dropDownBedsEl = document.getElementById("bed-dropdown");
  const dropDownBathsEl = document.getElementById("bath-dropdown");

  addEventListenerToDropdown(dropDownListingTypeEl, "listingType");
  addEventListenerToDropdown(dropDownLocationEl, "location");
  addEventListenerToDropdown(dropDownPropertyEl, "propertyType");
  addEventListenerToDropdown(dropDownBedsEl, "bed");
  addEventListenerToDropdown(dropDownBathsEl, "bath");

  handleClickOutsideDropdown(dropDownListingTypeEl, "listingType");
  handleClickOutsideDropdown(dropDownLocationEl, "location");
  handleClickOutsideDropdown(dropDownPropertyEl, "propertyType");
  handleClickOutsideDropdown(dropDownBedsEl, "bed");
  handleClickOutsideDropdown(dropDownBathsEl, "bath");
}

function addEventListenerToDropdown(dropDownContainerEl, type) {
  const dropDownEl =
    dropDownContainerEl.parentElement.querySelector(".dropdown");

  dropDownContainerEl.addEventListener("click", () => {
    hideShowDropdown(dropDownEl, type);
  });
}

function hideShowDropdown(dropDownEl, type) {
  let childNode =
    type === "listingType"
      ? dropDownEl.childNodes
      : dropDownEl.childNodes[0].childNodes;

  let height = 1.75 * childNode.length;

  if (type !== "listingType") {
    height = 2.1 * childNode.length;
  }

  if (type === "bed" || type === "bath") {
    height = 7;
  }

  if (childNode.length < 6 && type !== "listingType") {
    dropDownEl.classList.remove("overflow-auto");
    dropDownEl.classList.add("overflow-hidden");
  }

  if (type === "bed" || type === "bath") {
    dropDownEl.classList.remove("overflow-auto");
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
}

function handleClickOutsideDropdown(dropDownContainerEl, type) {
  document.addEventListener("click", (event) => {
    if (event.target === dropDownContainerEl) {
      return;
    }

    const dropDownEl =
      dropDownContainerEl.parentElement.querySelector(".dropdown");

    if (type !== "listingType" && dropDownEl.contains(event.target)) {
      return;
    }

    if (!dropDownContainerEl.contains(event.target)) {
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

export function setupPriceInput() {
  const princeInputMinEl = document.getElementById("price-label-min");
  const princeInputMaxEl = document.getElementById("price-label-max");

  if (princeInputMinEl && princeInputMaxEl) {
    princeInputMinEl.addEventListener("input", () => {});
  }
}

export function getListingTypeSelectValue() {
  const listingType = document.getElementById("listing-type-selection")?.value;

  return listingType;
}

export function setupChangeListingType() {
  const listingTypeSelectList = document.getElementById(
    "listing-type-select-list"
  );
  const listingTypeValue = document.getElementById("listing-type-selection");
  const listingTypeLabel = document.getElementById("listing-type-label");

  listingTypeSelectList.addEventListener("click", (event) => {
    if (event.target.id && event.target.id == "listing-type-select-list") {
      return;
    }

    if (!event.target.classList.contains("current-selected")) {
      const currentSelect =
        listingTypeSelectList.querySelector(".current-selected");
      const selectElement = event.target;

      currentSelect.classList.remove(
        "bg-blue-400",
        "text-white",
        "current-selected"
      );
      currentSelect.classList.add("hover:bg-blue-300", "hover:text-white");

      selectElement.classList.remove("hover:bg-blue-300", "hover:text-white");
      selectElement.classList.add(
        "bg-blue-400",
        "text-white",
        "current-selected"
      );

      listingTypeValue.value = selectElement.innerHTML
        .toLowerCase()
        .split(" ")
        .join("-");
      listingTypeLabel.textContent = selectElement.innerHTML;
    }
  });
}

window.getListingTypeSelectValue = getListingTypeSelectValue;
