# ⚙️ Zetra — AI-Powered SEO Crawler

**Zetra** is a high-performance SEO analysis engine built in Rust, designed to crawl client websites and extract valuable, AI-enhanced SEO insights. It uses vector embeddings, LLM analysis, and modern async concurrency to help users understand, optimize, and improve their on-page SEO at scale.

---

## 🚀 Features

- 🌐 **Multi-page Website Crawling**  
  Deeply crawls internal links using domain-based filtering and concurrency.

- 🧠 **AI SEO Analysis**  
  Uses GPT-4 / Gemini for SEO recommendations (title/meta fixes, keyword gaps, content ideas).

- 🔎 **Semantic Vector Storage**  
  Embeds page content using OpenAI/Gemini embeddings and stores in a vector database like Qdrant.

- 📊 **Per-Site Insights & Reporting**  
  Each client website is isolated with per-domain content, metadata, and AI output.

- ⚡ **Rust Performance**  
  Fully concurrent crawler written in Rust using `tokio`, optimized for deep crawling and high throughput.

---

