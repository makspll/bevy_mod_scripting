// Links the first mention of each glossary term on a page to its entry in `glossary.md`, with a hover card.
//
// The glossary page is the only source: each `##` entry is followed by an `<!-- aliases: a, b -->` comment
// listing the words to look for, and its first paragraph is the card's text.
// Code, links, headings and diagrams are left alone. Without the glossary page (i.e. opened from disk) nothing happens.
(function () {
    const GLOSSARY = "glossary.html";
    const SKIP = "pre, code, a, h1, h2, h3, h4, h5, h6, .mermaid, .bms-hero, .blockquote-tag-title, .multi-lang-example-tabs, .lad-page ~ *";

    function root() {
        return typeof path_to_root === "string" ? path_to_root : "";
    }

    function parseEntries(html) {
        const doc = new DOMParser().parseFromString(html, "text/html");
        const entries = [];
        const glossaryUrl = new URL(root() + GLOSSARY, window.location.href);
        function list(text) {
            return text ? text.split(",").map(function (a) { return a.trim().toLowerCase(); }).filter(Boolean) : [];
        }
        doc.querySelectorAll("main h2[id]").forEach(function (heading) {
            let node = heading.nextSibling;
            let aliases = null;
            let excluded = [];
            let summary = null;
            const pages = new Set();
            for (; node && !(node.nodeType === 1 && node.tagName === "H2"); node = node.nextSibling) {
                if (node.nodeType === 8 && aliases === null) {
                    // `aliases: a, b; not: phrase, phrase`
                    const match = node.textContent.match(/aliases:([^;]*)(?:;\s*not:(.*))?/);
                    if (match) {
                        aliases = list(match[1]);
                        excluded = list(match[2]);
                    }
                } else if (node.nodeType === 1) {
                    if (node.tagName === "P" && summary === null) {
                        summary = node;
                    }
                    // the pages the entry links to, a term isn't linked on its own pages
                    node.querySelectorAll("a[href]").forEach(function (a) {
                        pages.add(new URL(a.getAttribute("href"), glossaryUrl).pathname);
                    });
                }
            }
            if (aliases && summary) {
                entries.push({
                    id: heading.id,
                    title: heading.textContent.trim(),
                    aliases: aliases,
                    excluded: excluded,
                    pages: pages,
                    summary: summary.innerHTML,
                });
            }
        });
        return entries;
    }

    function escapeRegex(text) {
        return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    }

    function linkTerms(entries) {
        const main = document.querySelector("main");
        if (!main || entries.length === 0) {
            return;
        }
        const byAlias = new Map();
        entries.forEach(function (entry) {
            entry.aliases.forEach(function (alias) { byAlias.set(alias.toLowerCase(), entry); });
        });
        // longest first, so "context policy" wins over "context"
        const aliases = Array.from(byAlias.keys()).sort(function (a, b) { return b.length - a.length; });
        const pattern = new RegExp("\\b(" + aliases.map(escapeRegex).join("|") + ")\\b", "i");

        // terms aren't linked on the pages their entry points to
        const linked = new Set(entries.filter(function (e) {
            return e.pages.has(window.location.pathname);
        }).map(function (e) { return e.id; }));
        // whether the match at `index` is part of one of the entry's excluded phrases, i.e. "at runtime"
        function isExcluded(entry, value, index, length) {
            const lower = value.toLowerCase();
            return entry.excluded.some(function (phrase) {
                for (let at = lower.indexOf(phrase); at !== -1; at = lower.indexOf(phrase, at + 1)) {
                    if (at <= index && index + length <= at + phrase.length) {
                        return true;
                    }
                }
                return false;
            });
        }
        const walker = document.createTreeWalker(main, NodeFilter.SHOW_TEXT, {
            acceptNode: function (node) {
                const parent = node.parentElement;
                return parent && !parent.closest(SKIP) ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT;
            },
        });
        const nodes = [];
        while (walker.nextNode()) {
            nodes.push(walker.currentNode);
        }

        nodes.forEach(function (node) {
            let text = node;
            while (text) {
                // find the first alias in this text whose entry isn't linked yet
                let match = null;
                let offset = 0;
                const value = text.nodeValue;
                while (offset < value.length) {
                    const found = pattern.exec(value.slice(offset));
                    if (!found) {
                        break;
                    }
                    const entry = byAlias.get(found[1].toLowerCase());
                    const index = offset + found.index;
                    if (!linked.has(entry.id) && !isExcluded(entry, value, index, found[1].length)) {
                        match = { index: index, text: found[1], entry: entry };
                        break;
                    }
                    offset += found.index + found[1].length;
                }
                if (!match) {
                    break;
                }
                linked.add(match.entry.id);
                const term = text.splitText(match.index);
                const rest = term.splitText(match.text.length);
                const link = document.createElement("a");
                link.className = "glossary-term";
                link.href = root() + GLOSSARY + "#" + match.entry.id;
                link.dataset.glossary = match.entry.id;
                link.textContent = term.nodeValue;
                term.replaceWith(link);
                text = rest;
            }
        });
        return new Map(entries.map(function (e) { return [e.id, e]; }));
    }

    function setupCard(entries) {
        const card = document.createElement("div");
        card.className = "glossary-card";
        card.setAttribute("role", "tooltip");
        card.hidden = true;
        document.body.appendChild(card);

        let hideTimer = null;
        function show(link) {
            const entry = entries.get(link.dataset.glossary);
            if (!entry) {
                return;
            }
            clearTimeout(hideTimer);
            card.innerHTML = "";
            const title = document.createElement("div");
            title.className = "glossary-card-title";
            title.textContent = entry.title;
            const body = document.createElement("p");
            body.innerHTML = entry.summary;
            // links in the summary are relative to the glossary page, which sits at the book root
            body.querySelectorAll("a[href]").forEach(function (a) {
                const href = a.getAttribute("href");
                if (!/^(https?:|#|\/)/.test(href)) {
                    a.setAttribute("href", root() + href);
                }
            });
            const more = document.createElement("a");
            more.className = "glossary-card-more";
            more.href = link.href;
            more.textContent = "Glossary →";
            card.append(title, body, more);
            card.hidden = false;

            const rect = link.getBoundingClientRect();
            const width = card.offsetWidth;
            const left = Math.min(Math.max(8, rect.left), window.innerWidth - width - 8);
            const below = rect.bottom + card.offsetHeight + 8 < window.innerHeight;
            card.style.left = left + window.scrollX + "px";
            card.style.top = (below ? rect.bottom + 6 : rect.top - card.offsetHeight - 6) + window.scrollY + "px";
        }
        function hideSoon() {
            clearTimeout(hideTimer);
            hideTimer = setTimeout(function () { card.hidden = true; }, 150);
        }

        document.querySelectorAll("a.glossary-term").forEach(function (link) {
            link.addEventListener("mouseenter", function () { show(link); });
            link.addEventListener("focus", function () { show(link); });
            link.addEventListener("mouseleave", hideSoon);
            link.addEventListener("blur", hideSoon);
        });
        card.addEventListener("mouseenter", function () { clearTimeout(hideTimer); });
        card.addEventListener("mouseleave", hideSoon);
        document.addEventListener("keydown", function (e) {
            if (e.key === "Escape") {
                card.hidden = true;
            }
        });
    }

    document.addEventListener("DOMContentLoaded", function () {
        const page = window.location.pathname.split("/").pop();
        if (page === GLOSSARY || page === "print.html") {
            return;
        }
        fetch(root() + GLOSSARY)
            .then(function (response) { return response.ok ? response.text() : Promise.reject(response.status); })
            .then(function (html) {
                const entries = linkTerms(parseEntries(html));
                if (entries) {
                    setupCard(entries);
                }
            })
            .catch(function () {
                // no glossary, i.e. the book is opened from disk: leave the page as is
            });
    });
})();
