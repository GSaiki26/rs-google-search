# 🔎 Rust Google Search
[![Audit](https://github.com/GSaiki26/rs-google-search/actions/workflows/audit.yaml/badge.svg)](https://github.com/GSaiki26/rs-google-search/actions/workflows/audit.yaml) [![Linter](https://github.com/GSaiki26/rs-google-search/actions/workflows/linter.yaml/badge.svg)](https://github.com/GSaiki26/rs-google-search/actions/workflows/linter.yaml) [![Project build](https://github.com/GSaiki26/rs-google-search/actions/workflows/build.yaml/badge.svg)](https://github.com/GSaiki26/rs-google-search/actions/workflows/build.yaml)

The Rust Google Search is an exercise program written in Rust. It's a simple WebScraper that'll return the first 5 results from Google, allowing you to choose one. After selecting the source you want, it'll return the main content of the page.

# 🏃‍♂️ Running
In order to use the application, you may use Docker for simple deploy:
```sh
docker build -t gsaiki26/rs-google-search .;
docker run -ti --name google-search gsaiki26/rs-google-search "Why do cats purr?";
```
Or, just run using cargo:
```sh
cargo run -- "Why do cats purr?"
```

# 🛠️ Arguments
The basic CLI syntax is:
```sh
cargo run -- "TERM TO SEARCH" [...OPTIONS]
```

You can use theese flags:
<table>
    <tr>
        <th>Short</th>
        <th>Long</th>
        <th>Description</th>
    </tr>
    <tr>
        <td>-c</td>
        <td>None</td>
        <td>The css selector that'll be used to extract the main content from the source. By default, the main tag is used.</td>
    </tr>
    <tr>
        <td>-o</td>
        <td>--output</td>
        <td>The output path to write the content. If not defined, the stdout'll be used.</td>
    </tr>
    <tr>
        <td>-s</td>
        <td>-source</td>
        <td> The auto source to be selected. If not defined, the source'll be asked, requiring stdin.</td>
    </tr>
    <tr>
        <td>-h</td>
        <td>--help</td>
        <td>Print the help message.</td>
    </tr>
</table>

# 📋 To do
- [X] Create the rust workflows;
- [X] Request to Google;
- [X] Treat the response and get the links;
- [X] Asks the user by the wanted source;
- [X] Get all the main content of the selected source;
- [X] CLI flags: -h --help | -c | -o --output;
- [ ] Build the unit tests.
