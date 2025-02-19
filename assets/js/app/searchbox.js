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
    if (type === "location") {
      height = 2.1 * (childNode.length - 5);
    } else {
      height = 2.1 * childNode.length;
    }
  }

  if (type === "bed" || type === "bath") {
    height = 7;
  }

  if (childNode.length < 6 && type !== "listingType") {
    dropDownEl.classList.remove("overflow-auto");
    dropDownEl.classList.add("overflow-hidden");
  }

  if (childNode.length - 5 < 6 && type === "location") {
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
      "max-h-0",
      "duration-0"
    );

    dropDownEl.classList.add("opacity-100", "max-h-50", "duration-300");
    dropDownEl.style.height = `${height}rem`;
  } else {
    dropDownEl.style.height = 0;
    dropDownEl.classList.remove("opacity-100", "max-h-50", "duration-0");
    dropDownEl.classList.add(
      "invisible",
      "pointer-events-none",
      "opacity-0",
      "max-h-0",
      "duration-300"
    );
  }
}

const dropdownHandlers = new Map();

function handleClickOutsideDropdown(dropDownContainerEl, type) {
  function clickHandler(event) {
    if (event.target === dropDownContainerEl) {
      return;
    }

    const dropDownEl =
      dropDownContainerEl.parentElement.querySelector(".dropdown");

    if (type !== "listingType" && dropDownEl.contains(event.target)) {
      return;
    }

    if (!dropDownContainerEl.contains(event.target)) {
      if (type === "location") {
        const provinceVals = document.getElementById("province-vals");

        let currentProvinceDropdown = document.getElementById(
          `${provinceVals.value}-dropdown`
        );

        if (currentProvinceDropdown) {
          if (currentProvinceDropdown.classList.contains("flex")) {
            currentProvinceDropdown.classList.remove("flex");
            currentProvinceDropdown.classList.add("hidden");
          }
        }
      }

      dropDownEl.style.height = 0;
      dropDownEl.classList.remove("opacity-100", "max-h-50", "duration-0");
      dropDownEl.classList.add(
        "invisible",
        "pointer-events-none",
        "opacity-0",
        "max-h-0",
        "duration-300"
      );
    }
  }

  if (dropdownHandlers.has(`click_event_${type}`)) {
    console.log(dropdownHandlers.get(`click_event_${type}`));

    document.removeEventListener(
      "click",
      dropdownHandlers.get(`click_event_${type}`)
    );
  }
  dropdownHandlers.set(`click_event_${type}`, clickHandler);
  document.addEventListener("click", clickHandler);
}

export function setupPriceInput() {
  const princeInputMinEl = document.getElementById("price-label-min");
  const princeInputMaxEl = document.getElementById("price-label-max");

  if (princeInputMinEl && princeInputMaxEl) {
    princeInputMinEl.addEventListener("input", () => {});
  }
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

export function setupChangeLocation() {
  const provinceVals = document.getElementById("province-vals");
  const dropDownLocationEl = document.getElementById("location-dropdown");
  const dropDownEl =
    dropDownLocationEl.parentElement.querySelector(".dropdown");
  const locationLabel = document.getElementById("location-label");

  let childNode = dropDownEl.childNodes[0].childNodes;

  const locationDropdown = document.getElementById("location-dropdown-items");

  locationDropdown.addEventListener("click", (event) => {
    if (event.target.id && event.target.id == "location-dropdown-items") {
      return;
    }

    if (event.target.name === "province") {
      if (event.target.value !== provinceVals.value) {
        let currentProvinceDropdown = document.getElementById(
          `${provinceVals.value}-dropdown`
        );

        if (currentProvinceDropdown) {
          if (currentProvinceDropdown.classList.contains("flex")) {
            currentProvinceDropdown.classList.remove("flex");
            currentProvinceDropdown.classList.add("hidden");
          }
        }
      }

      let provinceDropdown = document.getElementById(
        `${event.target.value}-dropdown`
      );

      let provinceLocations = locationDropdown.querySelectorAll(
        `.${event.target.value}-child`
      );

      if (provinceDropdown) {
        if (provinceDropdown.classList.contains("hidden")) {
          if (childNode.length - 5 + provinceLocations.length < 6) {
            dropDownEl.classList.remove("overflow-auto");
            dropDownEl.classList.add("overflow-hidden");
          } else {
            dropDownEl.classList.remove("overflow-hidden");
            dropDownEl.classList.add("overflow-auto");
          }

          dropDownEl.classList.remove("duration-300");
          dropDownEl.classList.add("duration-0");
          dropDownEl.style.height = `${
            2.1 * (childNode.length + provinceLocations.length - 5)
          }rem`;

          provinceDropdown.classList.remove("hidden");
          provinceDropdown.classList.add("flex");
        } else {
          if (childNode.length - 5 < 6) {
            dropDownEl.classList.remove("overflow-auto");
            dropDownEl.classList.add("overflow-hidden");
          } else {
            dropDownEl.classList.remove("overflow-hidden");
            dropDownEl.classList.add("overflow-auto");
          }

          dropDownEl.classList.remove("duration-300");
          dropDownEl.classList.add("duration-0");
          dropDownEl.style.height = `${2.1 * (childNode.length - 5)}rem`;

          provinceDropdown.classList.remove("flex");
          provinceDropdown.classList.add("hidden");
        }
      }

      if (provinceVals.value === event.target.value) {
        return;
      }

      if (event.target.value === "All") {
        if (childNode.length - 5 + provinceLocations.length < 6) {
          dropDownEl.classList.remove("overflow-auto");
          dropDownEl.classList.add("overflow-hidden");
        } else {
          dropDownEl.classList.remove("overflow-hidden");
          dropDownEl.classList.add("overflow-auto");
        }

        dropDownEl.classList.remove("duration-300");
        dropDownEl.classList.add("duration-0");
        dropDownEl.style.height = `${
          2.1 * (childNode.length + provinceLocations.length - 5)
        }rem`;
      }

      let currentProvinceLocation = locationDropdown.querySelectorAll(
        `.${provinceVals.value}-child`
      );

      currentProvinceLocation.forEach((item) => {
        let locationInput = item.querySelector("input");
        locationInput.checked = false;
      });

      provinceVals.value = event.target.value;

      let countLocation = 0;

      provinceLocations.forEach((item) => {
        countLocation++;
        let locationInput = item.querySelector("input");
        locationInput.checked = true;
      });

      if (event.target.value === "All") {
        locationLabel.innerHTML = "All";
      } else {
        locationLabel.innerHTML = `${event.target.value}(${countLocation})`;
      }
    }
  });
}

export function getListingTypeSelectValue() {
  const listingType = document.getElementById("listing-type-selection")?.value;
  return listingType;
}

export function getProvinceValue() {
  const provinceVals = document.getElementById("province-vals")?.value;
  return provinceVals || "";
}

window.getListingTypeSelectValue = getListingTypeSelectValue;
window.getProvinceValue = getProvinceValue;
