console.log("Hello from data source");

const el = document.getElementById("data-json");

try {
  let json = JSON.parse(el.textContent);

  console.log(json);
} catch (error) {
  console.log(error);
}
