const input =
    document.getElementById(
        "searchInput"
    );

const button =
    document.getElementById(
        "searchBtn"
    );

const resultsDiv =
    document.getElementById(
        "results"
    );

const params =
    new URLSearchParams(
        window.location.search
    );

const initialQuery =
    params.get("q");

if (initialQuery) {

    input.value = initialQuery;

    search(initialQuery);
}

async function search(customQuery = null) {

    const query =
        customQuery ||
        input.value.trim();

    if (!query) return;

    resultsDiv.innerHTML =
        "<p>Loading...</p>";

    try {

        const response =
            await fetch(
                `http://localhost:3000/search?q=${
                    encodeURIComponent(query)
                }`
            );

        const results =
            await response.json();

        renderResults(results);

        // UPDATE URL
        window.history.pushState(
            {},
            "",
            `?q=${
                encodeURIComponent(query)
            }`
        );

    } catch (err) {

        resultsDiv.innerHTML =
            "<p>Error while searching.</p>";
    }
}

function renderResults(results) {

    resultsDiv.innerHTML = "";

    if (results.length === 0) {

        resultsDiv.innerHTML =
            "<p>No results found.</p>";

        return;
    }

    for (const result of results) {

        const div =
            document.createElement("div");

        div.className = "result";

        div.innerHTML = `

            <div class="source-badge">
                ${result.source}
            </div>

            <a href="${
                result.url
            }" target="_blank">

                ${result.title}

            </a>

            <p>
                ${result.snippet}
            </p>

        `;

        resultsDiv.appendChild(div);
    }
}

button.addEventListener(
    "click",
    () => search()
);

input.addEventListener(
    "keydown",
    (e) => {

        if (e.key === "Enter") {
            search();
        }
    }
);