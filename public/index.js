async function search(query, quantity, path) {
  const result = await fetch("/api/search", {
    method: "POST",
    body: JSON.stringify({
      path: path,
      query: query,
      quantity: quantity,
    }),
  }).then((data) => data.json());

  console.log(result);
  return result;
}

const input = document.querySelector("#query");
const searchButton = document.querySelector("#search");
const amount = document.querySelector("#amount");
const dataBuffer = document.querySelector("#data");
const path = document.querySelector("#path");
const dir = document.querySelector("#dir");
const preview = document.querySelector("#preview");

dir.addEventListener("click", async () => {
  const result = await fetch("/api/dialog", {
    method: "POST",
  }).then((data) => data.json());

  path.value = result.data;
});

searchButton.addEventListener("click", async () => {
  const query = input.value;
  const quantity = amount.value - 0;
  const dir_path = path.value;
  const result = await search(query, quantity, dir_path);

  const list = createListFromArray(result.data);
  dataBuffer.innerHTML = list;
});

function createListFromArray(array) {
  let html = ``;
  array.forEach((data) => {
    html += /*html*/ `<li>${data[0]} <button onclick="openInFile('${data[0]}')" >Open in file</button> <button onclick="showPreview('${data[0]}')">Preview</button> </li> `;
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

async function showPreview(path) {
  const result = await fetch("/api/preview", {
    method: "POST",
    body: JSON.stringify({
      path: path,
    }),
  }).then((data) => data.json());

  if (result.success == true) {
    preview.innerHTML = result.data;
  } else {
    preview.innerHTML = "Cannot give preview for this file";
  }
}
