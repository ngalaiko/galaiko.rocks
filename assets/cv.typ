#set text(font: "Berkeley Mono", size: 10pt)

// A4, 80 chars wide (6pt * 80 = 480pt)
#let margin-top = 1.8cm
#set page(
  paper: "a4",
  margin: (
    left: 57.64pt,
    right: 57.64pt,
    top: margin-top,
    bottom: 1.8cm,
  ),
  footer: align(center)[
    Page
    #context counter(page).display()
    of
    #context counter(page).final().first()
  ],
)
#set par(leading: 0.55em, spacing: 0.85em, justify: true)
#set smartquote(enabled: false)

#show heading.where(level: 1): it => {
  text(size: 10pt, weight: "regular", it.body)
}

#set list(marker: [-], spacing: 0.6em)
#show link: set text(fill: black)

// raw() + box() prevents hyphenation and line-breaking of URLs; raw defaults
// to a different mono font, so pin it to Berkeley Mono to match the body size
#let url(addr) = link(addr, raw(addr))
#show raw: set text(font: "Berkeley Mono", size: 10pt)
#show raw.where(block: false): box

// Skips rendering at the top of a page
#let sep() = context {
  let pos = here().position()
  if pos.y > margin-top + 0.7cm {
    block(width: 100%)[#repeat("=")]
  }
}

#let section-entry(
  number, company, location, body,
  role: none,
) = {
  let has-loc = location != none and location != []
  // "N.N. " is 5 monospace chars (1 char = 0.6em): the title hangs under the
  // company name at that width; the body/article is indented one char further.
  let indent = 3em
  v(1em)
  block(breakable: false, width: 100%)[
    #if has-loc [
      #par(hanging-indent: indent)[
        #{number} #{company} #box(width: 1fr, inset: (x: 0.4em))[#repeat(gap: 0.3em)[.]] #{location}
        #if role != none [#linebreak() #{role}]
      ]
    ] else [
      #{number} #{company}#{if role != none {" - "; role}}
      #align(right)[#location]
    ]
    #pad[#body]
  ]
}

#let section(title, body) = {
  block(breakable: false, width: 100%)[
    #sep()
    = #title
    #body
  ]
}

// Contacts sit in one grid so the right column shares a width; left-aligning
// it stacks the links on a common left edge (https:// prefixes line up).
#grid(
  columns: (1fr, auto),
  align: (left, left),
  row-gutter: 0.85em,
  [Nikita Galaiko], [nikita\@galaiko.rocks],
  [Software Engineer], [#url("https://github.com/ngalaiko")],
  [Göteborg, Sweden], [#url("https://nikita.galaiko.rocks")],
  [], [#url("https://linkedin.com/in/ngalaiko")],
)

#sep()

= 1. SUMMARY

Software engineer with 10 years of experience building and operating production
systems, focused on platform and developer tooling. I have been a founding /
early-stage engineer at three startups, taking products from concept to production
and maintenance. I am a core contributor to widely used open-source developer tools,
including GitButler (21,000+ GitHub stars) and the tree-sitter ecosystem (26,000+
stars). The work I enjoy the most is designing and building tools for other engineers.

#sep()

= 2. EMPLOYMENT HISTORY

#section-entry(role: [Founding Software Engineer])[2.1.][Cerve][Göteborg, Jan 2025 -- now][
Cerve builds infrastructure for the food industry: APIs, data collection from PDFs, and AI tooling. I joined as the first in-house engineer.

- Rebuilt a fragile, consultant-built platform into a clean, OpenAPI-driven production architecture, now the foundation of the product
- Migrated 6 live customer integrations to the new platform with no major disruptions
- Built the in-house engineering team from scratch and the data-ingestion and AI pipelines that turn customer documents into structured insights on GCP
]

#section-entry(role: [Founding Software Engineer])[2.2.][GitButler][Remote, Jan 2023 -- June 2024][
GitButler is a popular open-source Git client written in Rust, with 21,000+ GitHub stars. I joined as one of the first engineers.

- Kicked off the project and chose the tech stack (Rust, Tauri, Svelte, TypeScript)
- Built the first prototype of the cross-platform desktop app and the release process
- Designed the core "virtual branches" algorithm for working on multiple Git branches at once, built on libgit2
]

#section-entry(role: [Founding Software Engineer])[2.3.][Sturdy / Codeball][Stockholm, Sep 2021 -- Jan 2023][
Sturdy was an early-stage, open-source (500+ stars) startup building a real-time, cloud-based version control platform. I joined as one of the first engineers.

- Built a cross-platform desktop app syncing files in real time for live conflict detection, code review and GitHub integration
- Designed a single-codebase distribution strategy with open-source, cloud and enterprise editions
- After the pivot to Codeball (AI code review): built GitHub data scraping for model training, the infrastructure and the integration
]

#section-entry(role: [Software Engineer])[2.4.][Tink][Stockholm, Apr 2019 -- Sep 2021][
Tink is a European open-banking fintech that analyses bank transactions, later acquired by Visa.

- Built and maintained the company-wide API gateway and shared auth libraries for internal and external APIs in production
- Led the rate-limiting initiative and the internal libraries engineers used to adopt it
- Designed the migration of the main transaction store to improve reliability of a high-volume production system
]

#section-entry(role: [Software Engineer])[2.5.][Opera][Göteborg, Feb 2018 -- Apr 2019][
OPay is the payments product of Opera. I joined the core platform team before the public release.

- Rewrote the transaction-processing system to remove production bottlenecks and handle higher volume
- Built the integration framework other teams used to connect new payment processors
- Built a dynamic configuration system letting QA test configurations without redeploying
]

#section-entry(role: [Go Developer])[2.6.][Lazada][Moscow, Jun 2017 -- Jan 2018][
Lazada is a Southeast Asian e-commerce platform, acquired by Alibaba. I worked on the API Gateway team, keeping a high-traffic production system stable during sale campaigns.

- Built internal micro-services while breaking up a legacy PHP monolith
- After the Alibaba acquisition, spent two months in China on data-centre and Go-to-JVM migration
]

#section-entry(role: [Software Developer])[2.7.][TheQuestion / Yandex.Q][Moscow, Apr 2016 -- Jun 2017][
TheQuestion is a Q&A platform similar to Quora, later acquired by Yandex.

- Involved in every aspect of running the system: features, deployments and operations
- Built on-demand deployment of dev environments for specific versions, improving testing
]

#section[3. OPEN SOURCE PROJECTS][
#section-entry(role: [137 stars])[3.1.][#link("https://github.com/ngalaiko/tree-sitter-go-template")[tree-sitter-go-template]][][
Go template grammar for tree-sitter (ecosystem: 26,000+ stars), used by editors and language tooling. 137 stars, 6 contributors. (JavaScript, C)
]

#section-entry(role: [hardware, embedded])[3.2.][#link("https://github.com/ngalaiko/voop")[voop]][][
Screenless, zero-touch bike computer. Bare-metal Rust/Embassy firmware on an nRF52840 with BLE dual-role (reads a Garmin cadence sensor, streams over a custom GATT service), GPS/NMEA parsing and an OLED display; a SwiftUI iOS app derives rides and writes them to Apple Health. A shared Rust protocol crate generates the Swift wire-format bindings over FFI. (Rust, Embassy, Swift, BLE, C)
]

#section-entry(role: [Go, full-stack])[3.3.][#link("https://github.com/ngalaiko/miniboard")[miniboard]][][
Self-hosted RSS/Atom reader: a full-stack Go app with a web UI, authentication, background crawling and full-text search over SQLite. (Go, SQLite, JavaScript)
]

#section-entry(role: [Rust desktop app])[3.4.][#link("https://github.com/ngalaiko/hledger-desktop")[hledger-desktop]][][
Cross-platform desktop GUI for hledger plain-text accounting, built in Rust with Tauri and egui, with releases for macOS, Linux and Windows. (Rust, Tauri)
]

#section-entry[3.5.][#link("https://github.com/ngalaiko/cloudrun-local")[cloudrun-local]][][
Local proxy that emulates Google Cloud Run routing so engineers can run and test services locally. (Go, Docker, GCP)
]
]

#section[4. SKILLS][
- Programming languages: Golang, Rust, TypeScript, Java, Python, Swift, Bash, SQL
- Frontend frameworks: Svelte, Vue.js, React, SwiftUI
- Databases: PostgreSQL, MySQL, ElasticSearch, Redis, MongoDB, Spanner
- Cloud: Docker, Kubernetes, Terraform, Envoy, Nginx, AWS, GCP
- API design: REST, GraphQL, gRPC, OpenAPI
- Embedded: Rust/Embassy, nRF52840, BLE/GATT, GPS, no_std firmware
- Other: Git, libgit2, OAuth2, Tauri, Electron, Bazel, tree-sitter
]

#section[5. EDUCATION][
#section-entry(role: [Informatics and computer science])[5.1.][Higher School of Economics][Moscow, 2013 -- 2016][
Bachelor programme, incomplete.
]
]

#section[6. LANGUAGES][
- English (fluent)
- Swedish (beginner)
- Russian (native)
]
