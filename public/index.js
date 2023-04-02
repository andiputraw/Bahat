async function search(query) {
  const result = await fetch("/api/search", {
    method: "POST",
    body: query,
  });

  return await result.json();
}

const input = document.querySelector("#query");
const searchButton = document.querySelector("#search");
const amount = document.querySelector("#amount");
const dataBuffer = document.querySelector("#data");

searchButton.addEventListener("click", async () => {
  const query = input.value;
  const result = await search(query);
  const list = createListFromArray(result.data);
  dataBuffer.innerHTML = list;
});

function createListFromArray(array) {
  let html = ``;
  array.forEach((data) => {
    html += /*html*/ `<li>${data}</li>`;
  });
  return html;
}
