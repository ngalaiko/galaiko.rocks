async function importScripts() {
  await import("/scripts/leaflet.js");
}

async function importStyles() {
  // load leaflet css
  const leafletCSS = document.createElement("link");
  leafletCSS.rel = "stylesheet";
  leafletCSS.href = "/styles/leaflet.css";
  document.head.appendChild(leafletCSS);
}

function getPlaces() {
  const places = Array.from(document.querySelectorAll(".place"));
  function nodeToPlace(node) {
    return {
      name: node.dataset.name,
      latitude: parseFloat(node.dataset.latitude),
      longtitude: parseFloat(node.dataset.longtitude),
      visits: parseInt(node.dataset.visits),
      amount: parseFloat(node.dataset.amount),
      currency: node.dataset.currency,
    };
  }
  return places.map(nodeToPlace);
}

async function setupMap() {
  const map = document.querySelector("div#map");

  // only show map if js is enabled
  map.hidden = false;

  const places = getPlaces();

  // init map with gothenburg in the middle
  const m = L.map("map", {
    center: [57.704218, 11.969256],
    zoom: 13,
  });

  // use carto maps as a base layer
  L.tileLayer(
    "https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png",
    {
      attribution: `&copy;<a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>,
		    &copy;<a href="https://carto.com/attributions" target="_blank">CARTO</a>`,
      subdomains: "abcd",
      maxZoom: 16,
    },
  ).addTo(m);

  places.forEach((place) => {
    const marker = L.marker([place.latitude, place.longtitude], {
      icon: L.divIcon({
        html: document.createElement("div"),
        className: "icon",
      }),
      title: place.name,
      alt: place.name,
      riseOnHover: true,
    })
      .addTo(m)
      .bindPopup(
        [
          `<b>${place.name}</b>`,
          `visits: ${place.visits}`,
          `spent: ${place.amount} ${place.currency}`,
        ].join("<br/>"),
      );

    document
      .querySelector(`.place[data-name="${place.name}"]`)
      .addEventListener("click", () => {
        map.scrollIntoView();
        marker.openPopup();
        m.flyTo([place.latitude, place.longtitude]);
      });
  });
}

Promise.all([importScripts(), importStyles()]).then(setupMap);
