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

// raw() + box() prevents hyphenation and
// line-breaking of URLs
#let url(addr) = link(addr, raw(addr))
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
  v(1em)
  block(breakable: false, width: 100%)[
    #{number} #{company}#{if role != none {" - "; role}}
    #align(right)[#location]
    #body
  ]
}

#let section(title, body) = {
  block(breakable: false, width: 100%)[
    #sep()
    = #title
    #body
  ]
}

#grid(
  columns: (1fr, auto),
  [Nikita Galaiko],
  [nikita\@galaiko.rocks],
)
#grid(
  columns: (1fr, auto),
  [Software Engineer],
  [#url("https://github.com/ngalaiko")],
)
#align(right)[Göteborg, Sweden]

#sep()

= 1. SUMMARY

Software engineer with 10 years of experience and a platform engineering focus.
I work across the stack, bringing ideas from concept to production and maintenance.
The part that I enjoy the most is designing and building tools for other engineers.

#sep()

= 2. EMPLOYMENT HISTORY

#section-entry(role: [Founding Software Engineer])[2.1.][Cerve][Göteborg, Jan 2025 -- now][
Cerve is building infrastructure for food companies: APIs, data collection from PDFs and AI tooling on top.

- Rebuilt a fragile platform accumulated over years of consultant work, moving to an OpenAPI-driven architecture
- Migrated 6 live integrations to the new platform without major disruptions
- Built a small engineering in-house team to work on the project
- Currently building data ingestion and processing pipelines for data ingestion and AI harness to help customers get insights from their data
]

#section-entry(role: [Founding Software Engineer])[2.2.][GitButler][Remote, Jan 2023 -- June 2024][
GitButler builds modern git client (continued with the same team as Sturdy/Codeball, but a different product).

- Kicked off the project, choosing the tech stack
- Built a first prototype of the desktop app and setup development process
- Implemented first version of the core algorithm for working on multiple git branches simultaneously
]

#section-entry(role: [Founding Software Engineer])[2.3.][Sturdy / Codeball][Stockholm, Sep 2021 -- Jan 2023][
Sturdy was an early-stage startup building a real-time cloud-based version control platform.

- Built a desktop app syncing file changes to a remote server for real-time conflict detection, code review and GitHub integration
- Designed a distribution strategy with three flavours (open source, cloud and enterprise) from the same codebase with different features and licences
- After pivot to Codeball (an AI code review tool): built GitHub data scraping for model training, infrastructure, GitHub integration and a demo website
]

#section-entry(role: [Software Engineer])[2.4.][Tink][Stockholm, Apr 2019 -- Sep 2021][
Tink is a fintech that analyses bank transactions.

- Designed, built and maintained an API gateway and internal libraries for authentication/authorisation across internal and external APIs
- Part of the working group to set API guidelines and best practices across the company
- Led rate-limiting efforts and wrote set of internal libraries to allow engineers to implement it in their services
- Designed migration of the main transaction store to improve reliability and performance of the system
]

#section-entry(role: [Software Engineer])[2.5.][Opera][Göteborg, Feb 2018 -- Apr 2019][
OPay is Opera's payments product.
I joined before the public release as part of the core platform team.

- Rewrote the internal transaction processing system to remove processing bottlenecks
- Set up an internal framework that integration teams used to connect more payment processors
- Built a dynamic configuration system allowing QAs to easily test different configurations of the system without redeploying
]

#section-entry(role: [Go Developer])[2.6.][Lazada][Moscow, Jun 2017 -- Jan 2018][
Lazada is a Southeast Asian e-commerce platform.
I worked in the team responsible for the API Gateway, focused on stability and performance during high-load sale campaigns.

- Built a couple of internal micro-services as part of splitting up an old PHP monolithic application
- Implemented skeleton framework for writing new micro-services
- After acquisition by Alibaba, spent two months in China helping with data migration to another datacenter and tech stack migration from Go to JVM
]

#section-entry(role: [Software Developer])[2.7.][TheQuestion / Yandex.Q][Moscow, Apr 2016 -- Jun 2017][
TheQuestion is a Q&A platform similar to Quora.
Later acquired by Yandex.

- Involved in every aspect of running the system: developing features, operating deployments
- Built on-demand deployment of development environments for specific versions, improving testing efficiency
]

#section[3. NOTABLE OPEN SOURCE PROJECTS][
#section-entry(role: [136 stars])[3.1.][#link("https://github.com/ngalaiko/tree-sitter-go-template")[tree-sitter-go-template]][][
Golang template grammar for tree-sitter.
]

#section-entry(role: [21 stars])[3.2.][#link("https://github.com/ngalaiko/bazel-action")[bazel-action]][][
A GitHub Action to run Bazel commands.
]

#section-entry[3.3.][#link("https://github.com/ngalaiko/hledger-desktop")[hledger-desktop]][][
Desktop app for hledger.
]

#section-entry[3.4.][#link("https://github.com/ngalaiko/cloudrun-local")[cloudrun-local]][][
Local development proxy for Google Cloud Run.
]
]

#section[4. SKILLS][
- Programming languages: Golang, Rust, TypeScript, Java, Python, Bash, SQL
- Frontend frameworks: Svelte, Vue.js, React
- Databases: PostgreSQL, MySQL, ElasticSearch, Redis, MongoDB, Spanner
- Cloud: Docker, Kubernetes, Terraform, Envoy, Nginx, AWS, GCP
- API design: REST, GraphQL, gRPC, OpenAPI
- Observability: Prometheus, Grafana, OpenTelemetry
- Other: Git, libgit2, OAuth2, Tauri, Electron, Bazel
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
