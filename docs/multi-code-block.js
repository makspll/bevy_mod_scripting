// Groups consecutive ```multi-lang-example,<lang>``` code blocks into a single tabbed block.
//
// mdbook renders the info string `multi-lang-example,lua,scenario=x` as the classes
// `language-multi-lang-example lua scenario=x` on the `code` element.
// The chosen language is remembered, and switched on every group in the page at once.
(function () {
    const MARKER = "language-multi-lang-example";
    const STORAGE_KEY = "bms-multi-lang-example";
    // highlight.js languages to use for languages it doesn't know
    const HIGHLIGHT_AS = {};

    function languageOf(code) {
        for (const cls of code.classList) {
            if (cls !== MARKER && !cls.startsWith("language-") && !cls.includes("=") && cls !== "hljs") {
                return cls;
            }
        }
        return "text";
    }

    function highlight(code, language) {
        if (typeof hljs === "undefined") {
            return;
        }
        const as = HIGHLIGHT_AS[language] || language;
        if (!hljs.getLanguage(as)) {
            return;
        }
        code.innerHTML = hljs.highlight(code.textContent, { language: as }).value;
        code.classList.add("hljs");
    }

    function storedLanguage() {
        try {
            return localStorage.getItem(STORAGE_KEY);
        } catch (e) {
            return null;
        }
    }

    function storeLanguage(language) {
        try {
            localStorage.setItem(STORAGE_KEY, language);
        } catch (e) {
            // storage is optional
        }
    }

    function selectLanguage(language) {
        document.querySelectorAll(".multi-lang-example").forEach(function (group) {
            const has = group.querySelector('[data-language="' + language + '"]');
            if (!has) {
                return;
            }
            group.querySelectorAll("[data-language]").forEach(function (el) {
                el.classList.toggle("active", el.dataset.language === language);
            });
        });
    }

    function buildGroup(pres) {
        const group = document.createElement("div");
        group.className = "multi-lang-example";
        const tabs = document.createElement("div");
        tabs.className = "multi-lang-example-tabs";
        group.appendChild(tabs);
        pres[0].parentNode.insertBefore(group, pres[0]);

        pres.forEach(function (pre, i) {
            const code = pre.querySelector("code");
            const language = languageOf(code);
            highlight(code, language);

            const tab = document.createElement("button");
            tab.type = "button";
            tab.textContent = language;
            tab.dataset.language = language;
            tab.addEventListener("click", function () {
                storeLanguage(language);
                selectLanguage(language);
            });
            tabs.appendChild(tab);

            pre.dataset.language = language;
            group.appendChild(pre);
            if (i === 0) {
                tab.classList.add("active");
                pre.classList.add("active");
            }
        });
    }

    document.addEventListener("DOMContentLoaded", function () {
        const pres = Array.from(document.querySelectorAll("pre > code." + MARKER)).map(function (code) {
            return code.parentElement;
        });

        // consecutive siblings form one group
        let current = [];
        pres.forEach(function (pre) {
            const previous = current[current.length - 1];
            if (previous && previous.nextElementSibling !== pre) {
                buildGroup(current);
                current = [];
            }
            current.push(pre);
        });
        if (current.length > 0) {
            buildGroup(current);
        }

        const stored = storedLanguage();
        if (stored) {
            selectLanguage(stored);
        }
    });
})();
