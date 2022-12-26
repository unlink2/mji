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

use lazy_static::lazy_static;

use crate::mjimap::{MjiMap, MjiMapEntry};

lazy_static! {
    pub static ref GITMOJI: MjiMap = [
        MjiMapEntry::new("art", "\u{1f3a8}", "Improve structure"),
        MjiMapEntry::new("zap", "\u{26a1}\u{fe0f}", "Performance improvement"),
        MjiMapEntry::new("fire", "\u{1f525}", "Removed files"),
        MjiMapEntry::new("bug", "\u{1f41b}", "Fixed a bug"),
        MjiMapEntry::new("ambulance", "\u{1f691}\u{fe0f}", "Critical hotfix"),
        MjiMapEntry::new("sparkles", "\u{2728}", "New feature"),
        MjiMapEntry::new("memo", "\u{1f4dd}", "Add/Update documentation"),
        MjiMapEntry::new("rocket", "\u{1f680}", "Deploy"),
        MjiMapEntry::new("lipstick", "\u{1f484}", "UI update"),
        MjiMapEntry::new("tada", "\u{1f389}", "Begin a project"),
        MjiMapEntry::new("white-check", "\u{2705}", "Add, update or pass tests"),
        MjiMapEntry::new("lock", "\u{1f512}\u{fe0f}", "Fix security issue"),
        MjiMapEntry::new("closed-lock", "\u{1f510}", "Add or update secrets"),
        MjiMapEntry::new("bookmark", "\u{1f516}", "Release version or tag"),
        MjiMapEntry::new("rotating-light", "\u{1f6a8}", "Fix compiler/linter issues"),
        MjiMapEntry::new("construction", "\u{1f6a7}", "Work in progress"),
        MjiMapEntry::new("green-heart", "\u{1f49a}", "Fix CI Build"),
        MjiMapEntry::new("arrow-down", "\u{2b07}\u{fe0f}", "Downgrade dependencies"),
        MjiMapEntry::new("arrow-up", "\u{2b06}\u{fe0f}", "Upgrade dependencies"),
        MjiMapEntry::new("pushpin", "\u{1f4cc}", "Pin dependencies"),
        MjiMapEntry::new(
            "construction-worker",
            "\u{1f477}",
            "Update CI or build system"
        ),
        MjiMapEntry::new("chart-up", "\u{1f4c8}", "Add or update analytics code"),
        MjiMapEntry::new("recycle", "\u{267b}\u{fe0f}", "Refactor code"),
        MjiMapEntry::new("heavy-plus", "\u{2795}", "Add dependency"),
        MjiMapEntry::new("heavy-minus", "\u{2796}", "Remove a dependency"),
        MjiMapEntry::new("wrench", "\u{1f527}", "Add or update configuration files"),
        MjiMapEntry::new("hammer", "\u{1f528}", "Add or update development scripts"),
        MjiMapEntry::new(
            "globe-meridians",
            "\u{1f310}",
            "Internationalization and localization"
        ),
        MjiMapEntry::new("pencil", "\u{270f}\u{fe0f}", "Fix typos"),
        MjiMapEntry::new(
            "poop",
            "\u{1f4a9}",
            "Write bad code that needs to be improved"
        ),
        MjiMapEntry::new("rewind", "\u{23ea}\u{fe0f}", "Revert changes"),
        MjiMapEntry::new("twisted", "\u{1f500}", "Merge branches"),
        MjiMapEntry::new(
            "package",
            "\u{1f4e6}\u{fe0f}",
            "Add or update compiled files"
        ),
        MjiMapEntry::new(
            "alien",
            "\u{1f47d}\u{fe0f}",
            "Updated code due to external changes"
        ),
        MjiMapEntry::new("truck", "\u{1f69a}", "Move or rename a resource"),
        MjiMapEntry::new("page-up", "\u{1f4c4}", "Add or update license"),
        MjiMapEntry::new("boom", "\u{1f4a5}", "Breaking changes"),
        MjiMapEntry::new("bento", "\u{1f371}", "Add or update assets"),
        MjiMapEntry::new("wheelchair", "\u{267f}\u{fe0f}", "Improve accessibility"),
        MjiMapEntry::new("bulb", "\u{1f4a1}", "Add or update comments"),
        MjiMapEntry::new("beers", "\u{1f37b}", "Write code drunkenly"),
        MjiMapEntry::new("speech", "\u{1f4ac}", "Add or update text"),
        MjiMapEntry::new(
            "filebox",
            "\u{1f5c3}\u{fe0f}",
            "Perform database related changes"
        ),
        MjiMapEntry::new("sound", "\u{1f50a}", "Add or update logs"),
        MjiMapEntry::new("mute", "\u{1f507}", "Remove logs"),
        MjiMapEntry::new("busts", "\u{1f465}", "Add or update contributors"),
        MjiMapEntry::new("children", "\u{1f6b8}", "Improve user experience"),
        MjiMapEntry::new(
            "building-construction",
            "\u{1f3d7}\u{fe0f}",
            "Make architectural changes"
        ),
        MjiMapEntry::new("mobile-phone", "\u{1f4f1}", "Work on responsive design"),
        MjiMapEntry::new("clown", "\u{1f921}", "Mock things"),
        MjiMapEntry::new("egg", "\u{1f95a}", "Add or update an easter egg"),
        MjiMapEntry::new("see-no-evil", "\u{1f648}", "Update gitignore file"),
        MjiMapEntry::new("camera", "\u{1f4f8}", "Add or update snapshots"),
        MjiMapEntry::new("alembic", "\u{2697}\u{fe0f}", "Perform experiments"),
        MjiMapEntry::new("mag", "\u{1f50d}\u{fe0f}", "Improved SEO"),
        MjiMapEntry::new("label", "\u{1f3f7}\u{fe0f}", "Add or update types"),
        MjiMapEntry::new("seedling", "\u{1f331}", "Add or update seed files"),
        MjiMapEntry::new(
            "triangle-flag",
            "\u{1f6a9}",
            "Add, update, or remove feature flags"
        ),
        MjiMapEntry::new("goal", "\u{1f945}", "Catch errors"),
        MjiMapEntry::new("dizzy", "\u{1f4ab}", "Add or update animations"),
        MjiMapEntry::new(
            "waste",
            "\u{1f5d1}\u{fe0f}",
            "Deprecate code that needs to be cleaned up"
        ),
        MjiMapEntry::new(
            "passport",
            "\u{1f6c2}",
            "Work on code related to authorization"
        ),
        MjiMapEntry::new(
            "bandage",
            "\u{1fa79}",
            "Simple fix for a non-critical issue"
        ),
        MjiMapEntry::new("monocle", "\u{1f9d0}", "Data exploration/inspection"),
        MjiMapEntry::new("coffin", "\u{26b0}\u{fe0f}", "Remove dead code"),
        MjiMapEntry::new("test-tube", "\u{1f9ea}", "Add a failing test"),
        MjiMapEntry::new("necktie", "\u{1f454}", "Add or update business logic"),
        MjiMapEntry::new("stethoscope", "\u{1fa7a}", "Add or update a health check"),
        MjiMapEntry::new("bricks", "\u{1f9f1}", "Infrastructure related changes"),
        MjiMapEntry::new(
            "technologist",
            "\u{1f9d1}\u{200d}\u{1f4bb}",
            "Improve developer experience"
        ),
        MjiMapEntry::new(
            "money-with-wings",
            "\u{1f4b8}",
            "Add sponsorships or money related infrastructure"
        ),
        MjiMapEntry::new(
            "thread",
            "\u{1f9f5}",
            "Add or update code related to multithreading or concurrency"
        ),
    ]
    .iter()
    .map(|c| (c.name.clone(), c.clone()))
    .collect::<MjiMap>();
}
