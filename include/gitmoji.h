
#include "mjimap.h"

/*
Gitmoji list based on https://github.com/carloscuesta/gitmoji
MIT License

Copyright (c) 2016-2022 Carlos Cuesta

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
const MjiMapEntry MJI_MAP[] = {
    {"art", "\U0001f3a8", "Improve structure"},
    {"zap", "\u26a1\ufe0f", "Performance improvement"},
    {"fire", "\U0001f525", "Removed files"},
    {"bug", "\U0001f41b", "Fixed a bug"},
    {"ambulance", "\U0001f691\ufe0f", "Critical hotfix"},
    {"sparkles", "\u2728", "New feature"},
    {"memo", "\U0001f4dd", "Add/Update documentation"},
    {"rocket", "\U0001f680", "Deploy"},
    {"lipstick", "\U0001f484", "UI update"},
    {"tada", "\U0001f389", "Begin a project"},
    {"white-check", "\u2705", "Add, update or pass tests"},
    {"lock", "\U0001f512\ufe0f", "Fix security issue"},
    {"closed-lock", "\U0001f510", "Add or update secrets"},
    {"bookmark", "\U0001f516", "Release version or tag"},
    {"rotating-light", "\U0001f6a8", "Fix compiler/linter issues"},
    {"construction", "\U0001f6a7", "Work in progress"},
    {"green-heart", "\U0001f49a", "Fix CI Build"},
    {"arrow-down", "\u2b07\ufe0f", "Downgrade dependencies"},
    {"arrow-up", "\u2b06\ufe0f", "Upgrade dependencies"},
    {"pushpin", "\U0001f4cc", "Pin dependencies"},
    {"construction-worker", "\U0001f477", "Update CI or build system"},
    {"chart-up", "\U0001f4c8", "Add or update analytics code"},
    {"recycle", "\u267b\ufe0f", "Refactor code"},
    {"heavy-plus", "\u2795", "Add dependency"},
    {"heavy-minus", "\u2796", "Remove a dependency"},
    {"wrench", "\U0001f527", "Add or update configuration files"},
    {"hammer", "\U0001f528", "Add or update development scripts"},
    {"globe-meridians", "\U0001f310", "Internationalization and localization"},
    {"pencil", "\u270f\ufe0f", "Fix typos"},
    {"poop", "\U0001f4a9", "Write bad code that needs to be improved"},
    {"rewind", "\u23ea\ufe0f", "Revert changes"},
    {"twisted", "\U0001f500", "Merge branches"},
    {"package", "\U0001f4e6\ufe0f", "Add or update compiled files"},
    {"alien", "\U0001f47d\ufe0f", "Updated code due to external changes"},
    {"truck", "\U0001f69a", "Move or rename a resource"},
    {"page-up", "\U0001f4c4", "Add or update license"},
    {"boom", "\U0001f4a5", "Breaking changes"},
    {"bento", "\U0001f371", "Add or update assets"},
    {"wheelchair", "\u267f\ufe0f", "Improve accessibility"},
    {"bulb", "\U0001f4a1", "Add or update comments"},
    {"beers", "\U0001f37b", "Write code drunkenly"},
    {"speech", "\U0001f4ac", "Add or update text"},
    {"filebox", "\U0001f5c3\ufe0f", "Perform database related changes"},
    {"sound", "\U0001f50a", "Add or update logs"},
    {"mute", "\U0001f507", "Remove logs"},
    {"busts", "\U0001f465", "Add or update contributors"},
    {"children", "\U0001f6b8", "Improve user experience"},
    {"building-construction", "\U0001f3d7\ufe0f", "Make architectural changes"},
    {"mobile-phone", "\U0001f4f1", "Work on responsive design"},
    {"clown", "\U0001f921", "Mock things"},
    {"egg", "\U0001f95a", "Add or update an easter egg"},
    {"see-no-evil", "\U0001f648", "Update gitignore file"},
    {"camera", "\U0001f4f8", "Add or update snapshots"},
    {"alembic", "\u2697\ufe0f", "Perform experiments"},
    {"mag", "\U0001f50d\ufe0f", "Improved SEO"},
    {"label", "\U0001f3f7\ufe0f", "Add or update types"},
    {"seedling", "\U0001f331", "Add or update seed files"},
    {"triangle-flag", "\U0001f6a9", "Add, update, or remove feature flags"},
    {"goal", "\U0001f945", "Catch errors"},
    {"dizzy", "\U0001f4ab", "Add or update animations"},
    {"waste", "\U0001f5d1\ufe0f", "Deprecate code that needs to be cleaned up"},
    {"passport", "\U0001f6c2", "Work on code related to authorization"},
    {"bandage", "\U0001fa79", "Simple fix for a non-critical issue"},
    {"monocle", "\U0001f9d0", "Data exploration/inspection"},
    {"coffin", "\u26b0\ufe0f", "Remove dead code"},
    {"test-tube", "\U0001f9ea", "Add a failing test"},
    {"necktie", "\U0001f454", "Add or update business logic"},
    {"stethoscope", "\U0001fa7a", "Add or update a health check"},
    {"bricks", "\U0001f9f1", "Infrastructure related changes"},
    {"technologist", "\U0001f9d1\u200d\U0001f4bb",
     "Improve developer experience"},
    {"money-with-wings", "\U0001f4b8",
     "Add sponsorships or money related infrastructure"},
    {"thread", "\U0001f9f5",
     "Add or update code related to multithreading or concurrency"},
    MJI_MAP_END};
