// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

(() => {
    const darkThemes = ['ayu', 'navy', 'coal'];
    const lightThemes = ['light', 'rust'];

    const classList = document.getElementsByTagName('html')[0].classList;

    let lastThemeWasLight = true;
    for (const cssClass of classList) {
        if (darkThemes.includes(cssClass)) {
            lastThemeWasLight = false;
            break;
        }
    }

    // the BMS palette (see bms-theme.css)
    const themeVariables = lastThemeWasLight
        ? {
            darkMode: false,
            background: '#ffffff',
            primaryColor: '#f1eef7',
            primaryBorderColor: '#184375',
            primaryTextColor: '#1f2333',
            secondaryColor: '#e3ecf7',
            tertiaryColor: '#f6f4fa',
            lineColor: '#3f4561',
            textColor: '#1f2333',
            clusterBkg: '#faf9fc',
            clusterBorder: '#c8bddc',
            noteBkgColor: '#f6f1ea',
            noteBorderColor: '#594a42',
            edgeLabelBackground: '#ffffff',
        }
        : {
            darkMode: true,
            background: '#0e1626',
            primaryColor: '#18213a',
            primaryBorderColor: '#c8bddc',
            primaryTextColor: '#e6e2ef',
            secondaryColor: '#3f4561',
            tertiaryColor: '#141e33',
            lineColor: '#c8bddc',
            textColor: '#e6e2ef',
            clusterBkg: '#111a2d',
            clusterBorder: '#3f4561',
            noteBkgColor: '#594a42',
            noteBorderColor: '#d4b89c',
            noteTextColor: '#ffffff',
            edgeLabelBackground: '#0e1626',
        };
    mermaid.initialize({
        // rendered once the fonts are loaded, as mermaid sizes the nodes to the text
        startOnLoad: false,
        theme: 'base',
        // allows `click` links between diagrams, scripts are still stripped
        securityLevel: 'antiscript',
        flowchart: { wrappingWidth: 320 },
        themeVariables: { fontFamily: 'Inter, sans-serif', ...themeVariables },
    });
    Promise.all([document.fonts.load('16px Inter'), document.fonts.load('italic 16px Inter')])
        .catch(() => {})
        .then(() => mermaid.run());

    // Simplest way to make mermaid re-render the diagrams in the new theme is via refreshing the page

    // the theme buttons are `mdbook-theme-<theme>` since mdbook 0.5, and `<theme>` before
    const themeButton = (theme) =>
        document.getElementById(`mdbook-theme-${theme}`) || document.getElementById(theme);

    for (const darkTheme of darkThemes) {
        themeButton(darkTheme)?.addEventListener('click', () => {
            if (lastThemeWasLight) {
                window.location.reload();
            }
        });
    }

    for (const lightTheme of lightThemes) {
        themeButton(lightTheme)?.addEventListener('click', () => {
            if (!lastThemeWasLight) {
                window.location.reload();
            }
        });
    }
})();
