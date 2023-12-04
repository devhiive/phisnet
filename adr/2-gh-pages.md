# ADR for Hosting Using GitHub Pages

## Title

Adopting GitHub Pages for Project Hosting

## Status

Proposed

## Context

We are considering different hosting options for our project's website, which primarily consists of static pages. Our needs include ease of deployment, reliability, and cost-effectiveness. GitHub Pages emerges as a potential solution, offering integration with our existing GitHub workflows and repositories.

## Decision

We propose to use GitHub Pages for hosting our project's website. This platform allows us to deploy directly from our repository, providing a streamlined process from code changes to deployment. GitHub Pages supports Jekyll for site generation, enabling us to use markdown files and simple templates to create our site content. Additionally, it offers free hosting, which is a significant advantage for our budget.

## Consequences

**Positive Consequences:**
1. Simplified deployment process, with automatic updates directly from the GitHub repository.
2. No additional cost for hosting, as GitHub Pages is free for public repositories.
3. Seamless integration with other GitHub features and workflows, enhancing collaboration and version control.
4. Reduced time to market, as there is no need to manage separate hosting infrastructure.

**Negative Consequences:**
1. Limited to static content, restricting our ability to host dynamic web applications.
2. Restricted control over server configuration and capabilities.
3. Potential scalability issues for high-traffic websites.
4. Dependence on GitHub's infrastructure and uptime.

By choosing GitHub Pages, we aim to leverage its integration with our existing tools and workflows, while acknowledging its limitations in terms of dynamic content and server control.
