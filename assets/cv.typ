// Font setup - Berkeley Mono, 10pt
#set text(font: "Berkeley Mono", size: 10pt)

// Page setup - A4, 80 chars wide
// (6pt * 80 = 480pt, margin = (210mm - 480pt) / 2)
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

// Disable heading default styling
// - just plain text with a blank line around it
#show heading.where(level: 1): it => {
  text(size: 10pt, weight: "regular", it.body)
}

// List styling
#set list(marker: [-], spacing: 0.6em)

// Disable link colouring
#show link: set text(fill: black)

// URL helper - uses raw() + box() to prevent
// hyphenation and line-breaking of URLs
#let url(addr) = {
  link(addr, raw(addr))
}
#show raw.where(block: false): it => {
  set text(font: "Berkeley Mono", size: 10pt)
  box(it)
}

// Separator - text-based, full width.
// Skips rendering if at the top of a page
// (page break is a natural separator).
#let sep() = context {
  let pos = here().position()
  if pos.y > margin-top + 0.7cm {
    block(width: 100%)[#repeat("=")]
  }
}

// Job entry - header + description,
// always kept together on one page
#let job-entry(
  number, company, role, location, body,
) = {
  v(1em)
  block(breakable: false, width: 100%)[
    #number #company - #role
    #align(right)[#location]
    #body
  ]
}

// Section - separator + heading + body,
// always kept together on one page
#let section(title, body) = {
  block(breakable: false, width: 100%)[
    #sep()
    = #title
    #body
  ]
}

// Header
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

Platform engineer at heart with 10 years of
experience. I build the foundations that empower
teams to ship faster and more reliably, but I'm
no stranger to business logic and frontend work
too. More recently I've spent a lot of time deep
in git internals. I'm passionate about developer
experience, and don't feel restricted by
languages or tools - it's all just code. I prefer
keeping things simple, slightly functional, with
trunk-based development and continuous
deployments.

#sep()

= 2. EMPLOYMENT HISTORY

#job-entry[2.1.][Cerve][
  Founding Software Engineer
][Göteborg, Jan 2025 -- now][
Cerve is building infrastructure for food
companies: APIs, data collection from PDFs and AI
tooling on top.

- Rebuilt a fragile platform accumulated over
  years of consultant work, moving to an
  OpenAPI-driven architecture
- Migrated 6 live integrations (around 1k daily
  transactions) to the new platform without major
  disruptions
- Hired a small in-house team to work on the
  project
- Currently building data ingestion and processing
  pipelines and infrastructure for the AI layer
]

#job-entry[2.2.][GitButler][
  Founding Software Engineer
][Remote, Jan 2023 -- June 2024][
GitButler is an early-stage startup building a
modern git client, from the same team behind
Sturdy.

- Chose the app tech stack and set up the
  development foundation, which is still used by
  the team today
- Built a desktop app tracking repository state
  and file changes, and implemented the core
  algorithm for working on multiple git branches
  simultaneously
]

#job-entry[2.3.][Sturdy / Codeball][
  Founding Software Engineer
][Stockholm, Sep 2021 -- Jan 2023][
Sturdy was an early-stage startup building a
real-time cloud-based version control platform.

- Built a desktop app syncing file changes to a
  remote server for real-time conflict detection,
  code review and GitHub integration
- Designed a distribution strategy with three
  flavours (open source, cloud and enterprise)
  from the same codebase with different features
  and licences
- After pivot to Codeball (an AI code review
  tool): built GitHub data scraping for model
  training, infrastructure, GitHub integration
  and a demo website
]

#job-entry[2.4.][Tink][
  Software Engineer
][Stockholm, Apr 2019 -- Sep 2021][
Tink is a fintech that analyses bank
transactions.

- Designed, built and maintained an API gateway
  and internal libraries for
  authentication/authorisation across internal
  and external APIs
- Led rate-limiting efforts and wrote an internal
  Java auth library
- Designed and executed a zero-downtime migration
  of the main transaction store from
  ElasticSearch to Cassandra using a
  double-write/switchover strategy
- Designed guidelines for public APIs and built
  tools to help other teams adopt them
]

#job-entry[2.5.][Opera][
  Software Engineer
][Göteborg, Feb 2018 -- Apr 2019][
OPay is Opera's payments product. I joined before
the public release as part of the core platform
team.

- Rewrote the internal transaction processing
  system to remove processing bottlenecks
- Set up an internal framework that integration
  teams used to connect more payment processors
- Built a dynamic configuration system allowing
  testers to override settings per session for
  scenario testing
]

#job-entry[2.6.][Lazada][
  Go Developer
][Moscow, Jun 2017 -- Jan 2018][
Lazada is a Southeast Asian e-commerce platform.
I worked in the team responsible for the API
Gateway, focused on stability and performance
during high-load sale campaigns.

- Built internal micro-services as part of
  splitting up an old PHP monolithic application
- Implemented a skeleton framework for starting
  new micro-services
- After acquisition by Alibaba, spent two months
  in China helping with data migration to another
  datacenter and tech stack migration from Go to
  JVM
]

#job-entry[2.7.][TheQuestion / Yandex.Q][
  Software Developer
][Moscow, Apr 2016 -- Jun 2017][
TheQuestion is a Q&A platform similar to Quora,
later acquired by Yandex.

- Involved in every aspect of running the system
  - developing features, operating deployments
- Built on-demand deployment of development
  environments for specific versions, improving
  testing efficiency
]

#section[3. SKILLS][
- Programming languages: Golang, Rust,
  TypeScript, Java, Python, Bash, SQL
- Frontend frameworks: Svelte, Vue.js, React
- Databases: PostgreSQL, MySQL, ElasticSearch,
  Redis, MongoDB, Spanner
- Cloud: Docker, Kubernetes, Terraform, Envoy,
  Nginx, AWS, GCP
- API design: REST, GraphQL, gRPC, OpenAPI
- Observability: Prometheus, Grafana,
  OpenTelemetry
- Other: Git, libgit2, OAuth2, Tauri, Electron,
  Bazel
]

#section[4. EDUCATION][
#job-entry[4.1.][Higher School of Economics][
  Informatics and computer science
][Moscow, 2013 -- 2016][
Bachelor programme, incomplete.
]
]

#section[5. LANGUAGES][
English (fluent), Swedish (beginner),
Russian (native)
]
