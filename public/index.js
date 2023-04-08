async function search(query, quantity) {
  const result = await fetch("/api/search", {
    method: "POST",
    body: JSON.stringify({
      query: query,
      quantity: quantity,
    }),
  });
  return await result.json();
  j;
}

const input = document.querySelector("#query");
const searchButton = document.querySelector("#search");
const amount = document.querySelector("#amount");
const dataBuffer = document.querySelector("#data");

searchButton.addEventListener("click", async () => {
  const query = input.value;
  const quantity = amount.value - 0;
  const result = await search(query, quantity);

  const list = createListFromArray(result.data);
  dataBuffer.innerHTML = list;
});

function createListFromArray(array) {
  let html = ``;
  array.forEach((data) => {
    html += /*html*/ `<li>${data[0]} <button onclick="openInFile('${data[0]}')" >Open in file</button> </li> `;
  });
  return html;
}

async function openInFile(path) {
  const result = await fetch("/api/open", {
    method: "POST",
    body: JSON.stringify({
      path: path,
    }),
  });

  console.log(await result.json());
}
