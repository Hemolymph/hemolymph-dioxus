import init, { search_wasm, parse_wasm } from "../hemoglobin/hemoglobin_search.js";

function debounce<F extends (...args: any[]) => void>(
  fn: F,
  delay: number
): (...args: Parameters<F>) => void {
  let timer: ReturnType<typeof setTimeout> | undefined;

  return (...args) => {
    if (timer !== undefined) {
      clearTimeout(timer);
    }
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  };
}

let params = new URLSearchParams(window.location.search);
const frag = document.createDocumentFragment();
let search: HTMLInputElement | undefined;
let results: HTMLDivElement | undefined;
let display: HTMLDivElement | undefined;
let cards: any[];


document.addEventListener("DOMContentLoaded", async () => {
  await init();
  search = document.getElementById("search") as HTMLInputElement;
  results = document.getElementById("results") as HTMLDivElement;
  display = document.getElementById("display") as HTMLDivElement;
  const q = params.get("q");
  if (search != undefined && q != null && q.trim().length > 0) {
    search.value = q;
  }
  await loadCardData();
  const c = params.get("c");
  if (c == null || c.trim().length == 0) {
    try {
      searchCards(search.value);
    } catch {
      searchCards("")
    }
  } else {
    try {
      displayCard(c);
    } catch {
      searchCards(search.value);
    }
  }
  search.addEventListener("input", debounce(onInput, 0))
  results.appendChild(frag);
  window.addEventListener("popstate", (_evt) => {
    params = new URLSearchParams(window.location.search);
    if (search == undefined) {
      return
    }
    const q = params.get("q");
    if (q != null && q.trim().length > 0) {
      search.value = q;
    }
    const c = params.get("c");

    if (c == null || c.trim().length == 0) {
      searchCards(search.value);
    } else {
      displayCard(c);
    }
  })
});

function onInput(_evt: Event): void {
  if (search == undefined || results == undefined) {
    return
  }

  let value = search.value;
  params.set("q", value);
  params.delete("c");
  history.pushState(null, "", `?${params.toString()}`)

  searchCards(value);
}

function displayCard(id: string | undefined) {
  let card = cards.find((val) => { return val.id == id });
  if (card == undefined || results == undefined || display == undefined) {
    throw new Error(`Page wasn't loaded maybe ${display} ${card} ${results}`);
  }

  results.replaceChildren();
  display.replaceChildren();

  let cname = card.images[0].sources[0];
  const left = document.createElement("img");
  left.id = "display-left";
  left.src = "https://static.hemolymph.net/cardimgs/" + cname + ".png"

  const right = document.createElement("div");
  right.id = "display-right";

  const name = document.createElement("p");
  name.id = "display-name";
  name.textContent = card.name;
  right.appendChild(name);

  const typecostline = document.createElement("p");
  typecostline.textContent = `${card.type} :: ${card.cost} Blood`;
  right.appendChild(typecostline);
  right.appendChild(document.createElement("hr"));

  const description = document.createElement("div");
  description.id = "display-desc";


  description.appendChild(displayText(card.description_rich));

  right.appendChild(description);
  if (card.flavor_text != undefined && card.flavor_text != "") {
    const flavor_section = document.createElement("div");
    flavor_section.id = "display-flavor";
    for (let s of card.flavor_text.split("\n")) {
      const flavor_line = document.createElement("p");
      flavor_line.textContent = s;
      flavor_section.appendChild(flavor_line);
    }
    right.appendChild(document.createElement("hr"));
    right.appendChild(flavor_section);
  }
  if (!card.type.includes("command")) {
    const statline = document.createElement("p");
    statline.textContent = `${card.health}/${card.defense}/${card.power}`;
    right.appendChild(document.createElement("hr"));
    right.appendChild(statline);
  }

  display.appendChild(left);
  display.appendChild(right);
}

function displayText(text: any): DocumentFragment {
  let content = document.createDocumentFragment();
  let lbreaks = 0;
  let current_p = document.createElement("p");
  if (text == undefined) {
    return content;
  }
  for (let el of text.elements) {
    if (el != "LineBreak") {
      lbreaks = 0;
    }
    if (el.String != undefined) {
      let text = document.createTextNode(el.String);
      current_p.appendChild(text);
    } else if (el.Saga != undefined) {
      if (current_p.childNodes.length != 0) {
        content.appendChild(current_p);
        current_p = document.createElement("p");
      }

      let list = document.createElement("ol");
      for (let sel of el.Saga) {
        let listel = document.createElement("li");
        listel.appendChild(displayText(sel));
        list.appendChild(listel);
      }
      content.appendChild(list)
    } else if (el == "LineBreak") {
      if (lbreaks > 0) {
        content.appendChild(document.createElement("br"));
      } else if (current_p.childNodes.length != 0) {
        content.appendChild(current_p);
        current_p = document.createElement("p");
      }
      lbreaks += 1;
    } else if (el.CardSearch != undefined) {
      let hemolink = document.createElement("button");
      hemolink.className = "hemolink";
      hemolink.textContent = el.CardSearch.display;
      hemolink.addEventListener("click", (_evt) => {
        if (search != undefined) {
          search.value = el.CardSearch.search;
        }
        params.set("q", el.CardSearch.search);
        params.delete("c");
        history.pushState(null, "", `?${params.toString()}`)
        searchCards(el.CardSearch.search);
      });
      current_p.appendChild(hemolink);
    }
  }

  if (current_p.childNodes.length != 0) {
    content.appendChild(current_p);
  }

  return content
}

// function searchCards(string: string): void {
//   if (search == undefined || results == undefined) {
//     return
//   }

//   let found = sch.search(string, cards)

//   if (display != undefined) {
//     display.replaceChildren();
//   }
//   results.replaceChildren();
//   frag.replaceChildren();

//   for (let match of found) {
//     const root = document.createElement("div");
//     root.className = "result";

//     let cname = match.images[0].sources[0];

//     const img = document.createElement("img");
//     img.className = "result-img";
//     img.loading = "lazy";
//     img.decoding = "async";
//     img.src = "https://static.hemolymph.net/cardimgs/" + cname + ".png"

//     const button = document.createElement("button");
//     button.className = "result-btn";
//     button.addEventListener("click", (_evt) => {
//       params.set("c", match.id);
//       history.pushState(null, "", `?${params.toString()}`)
//       displayCard(match.id);
//     });

//     button.appendChild(img)

//     const name = document.createElement("span");
//     name.className = "result-name";
//     name.textContent = match.name;

//     root.append(button, name);
//     frag.appendChild(root);
//   }

//   results.appendChild(frag);
// }
function searchCards(string: string): void {
  if (search == undefined || results == undefined) {
    return
  }

  let query = parse_wasm(string);
  console.log(query)
  let found = search_wasm(string, cards);
  console.log(found)

  if (display != undefined) {
    display.replaceChildren();
  }

  for (let match of found) {
    const root = document.createElement("div");
    root.className = "result";

    let cname = match.images[0].sources[0];

    const img = document.createElement("img");
    img.className = "result-img";
    img.loading = "lazy";
    img.decoding = "async";
    img.src = "https://static.hemolymph.net/cardimgs/" + cname + ".png"

    const button = document.createElement("button");
    button.className = "result-btn";
    button.addEventListener("click", (_evt) => {
      params.set("c", match.id);
      history.pushState(null, "", `?${params.toString()}`)
      displayCard(match.id);
    });

    button.appendChild(img)

    const name = document.createElement("span");
    name.className = "result-name";
    name.textContent = match.name;

    root.append(button, name);
    frag.appendChild(root);
  }

  results.replaceChildren(frag);
}

async function loadCardData() {
  let cards_response = await fetch("https://static.hemolymph.net/cards-Rrx.json");
  cards = await cards_response.json();
  cards.sort((a, b) => {
    return a.name.localeCompare(b.name)
  })
}






