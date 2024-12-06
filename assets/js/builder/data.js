export function checkDataFormDropdown() {
  const createDataFormEl = document.getElementById("create-data-form");
  const rsoDataCheckBoxEl = document.getElementById("rso-data-status");

  function toggleCreateDataFormDisplay() {
    if (createDataFormEl && rsoDataCheckBoxEl) {
      if (rsoDataCheckBoxEl.checked == false) {
        rsoDataCheckBoxEl.value = "false";
        createDataFormEl.classList.remove("flex");
        createDataFormEl.classList.add("hidden");
      } else {
        rsoDataCheckBoxEl.value = "true";
        createDataFormEl.classList.remove("hidden");
        createDataFormEl.classList.add("flex");
      }
    }
  }

  toggleCreateDataFormDisplay();

  rsoDataCheckBoxEl.addEventListener("change", (_) => {
    toggleCreateDataFormDisplay();
  });
}
