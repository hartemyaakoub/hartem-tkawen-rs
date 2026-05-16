use maud::{html, Markup, PreEscaped, DOCTYPE};
use crate::content as c;
use crate::i18n::Lang;

const CSS: &str = include_str!("../assets/styles.css");

pub fn page(lang: Lang, render_us: u128) -> Markup {
    let title = match lang {
        Lang::En => "Hartem Yaakoub — Engineering the digital future",
        Lang::Fr => "Hartem Yaakoub — Architecte de l'avenir numérique",
        Lang::Ar => "حرتام يعقوب — مهندس المستقبل الرقمي",
    };
    let desc = match lang {
        Lang::En => "Engineer and founder of TKAWEN. Eleven production digital platforms. Recognized by MESRS, the Ministry of Knowledge Economy, ASEP, and Algeria Venture.",
        Lang::Fr => "Ingénieur et fondateur de TKAWEN. Onze plateformes en production. Reconnu par le MESRS, le Ministère de l'Économie de la Connaissance, ASEP, et Algeria Venture.",
        Lang::Ar => "مهندس ومؤسس تكوّن. إحدى عشرة منصّة رقميّة إنتاجيّة. معترَف به من وزارة التعليم العالي، وزارة اقتصاد المعرفة، ASEP، Algeria Venture.",
    };

    html! {
        (DOCTYPE)
        html lang=(lang.code()) dir=(lang.dir()) {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover";
                title { (title) }
                meta name="description" content=(desc);
                meta name="robots" content="index,follow,max-image-preview:large";
                link rel="canonical" href="https://hartem.tkawen.com/";
                link rel="alternate" hreflang="en" href="https://hartem.tkawen.com/";
                link rel="alternate" hreflang="fr" href="https://hartem.tkawen.com/?lang=fr";
                link rel="alternate" hreflang="ar" href="https://hartem.tkawen.com/?lang=ar";
                link rel="alternate" hreflang="x-default" href="https://hartem.tkawen.com/";
                meta property="og:type" content="profile";
                meta property="og:title" content=(title);
                meta property="og:description" content=(desc);
                meta property="og:image" content="https://hartem.tkawen.com/img/portrait.jpg";
                meta property="og:url" content="https://hartem.tkawen.com/";
                meta property="og:site_name" content="Hartem Yaakoub";
                meta property="og:locale" content=(lang.locale());
                meta property="profile:first_name" content="Yaakoub";
                meta property="profile:last_name" content="Hartem";
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:image" content="https://hartem.tkawen.com/img/portrait.jpg";
                meta name="theme-color" content="#0a0908";
                link rel="icon" href="/favicon.svg";
                link rel="apple-touch-icon" href="/img/portrait.jpg";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Fraunces:ital,opsz,wght@0,9..144,300;0,9..144,400;0,9..144,500;0,9..144,600;0,9..144,700;1,9..144,300;1,9..144,400&family=Inter:wght@300;400;500;600;700&family=Cairo:wght@300;400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600&display=swap";
                style { (PreEscaped(CSS)) }

                // ===== JSON-LD =====
                script type="application/ld+json" { (PreEscaped(crate::schemas::person())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::profile_page())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::website())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::solutions_itemlist())) }
            }
            body lang=(lang.code()) {
                div.grain {}
                div.aura {}

                // MASTHEAD
                header.masthead {
                    div.wrap {
                        div.row {
                            div.issue { (c::ISSUE.get(lang)) }
                            div.title { "Hartem " em { "Yaakoub" } }
                            div.lang {
                                a href="/?lang=en" class=(if matches!(lang, Lang::En) { "cur" } else { "" }) { "EN" }
                                a href="/?lang=fr" class=(if matches!(lang, Lang::Fr) { "cur" } else { "" }) { "FR" }
                                a href="/?lang=ar" class=(if matches!(lang, Lang::Ar) { "cur" } else { "" }) { "AR" }
                            }
                        }
                    }
                }

                // HERO COVER
                section.cover {
                    div.wrap {
                        div.grid {
                            div.left {
                                div.kicker { (c::HERO_KICKER.get(lang)) }
                                h1 { (PreEscaped(c::HERO_H1.get(lang))) }
                                p.deck { (PreEscaped(c::HERO_DECK.get(lang))) }
                                div.byline {
                                    span { b { "11" } " " (match lang { Lang::En => "platforms", Lang::Fr => "plateformes", Lang::Ar => "منصّة" }) }
                                    span.sep { "·" }
                                    span { b { "3" } " " (match lang { Lang::En => "languages", Lang::Fr => "langues", Lang::Ar => "لغات" }) }
                                    span.sep { "·" }
                                    span { b { "22" } " " (match lang { Lang::En => "open repos", Lang::Fr => "dépôts ouverts", Lang::Ar => "مستودع مفتوح" }) }
                                    span.sep { "·" }
                                    span { b { "ASEP" } " Cohort 3 · 🇰🇷" }
                                    span.sep { "·" }
                                    span { b { "D-U-N-S" } " 35-355-1313" }
                                }
                            }
                            div.right {
                                div.portrait-frame {
                                    div.badge { "Founder · TKAWEN" }
                                    img src="/img/portrait.jpg" alt="Hartem Yaakoub — portrait";
                                    div.caption {
                                        div.nm { (match lang { Lang::Ar => "حرتام يعقوب", _ => "Hartem Yaakoub" }) }
                                        div.sub { "State Engineer · HSI · UBMA" }
                                    }
                                }
                            }
                        }
                    }
                }

                // ENDORSE
                section.endorse {
                    div.wrap {
                        div.label { (c::ENDORSE_LABEL.get(lang)) }
                        div.logos {
                            @for p in c::PARTNERS {
                                img src={ "/img/partners/" (p.file) } alt=(p.alt) title=(p.title);
                            }
                        }
                    }
                }

                // STORY
                section.section id="story" {
                    div.wrap-mid {
                        div.section-num { "N° 01" }
                        div.kicker-2 { (c::STORY_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::STORY_H2.get(lang))) }
                        p.deck { (PreEscaped(c::STORY_DECK.get(lang))) }

                        div.editorial {
                            p { (PreEscaped(c::STORY_P1.get(lang))) }
                            p { (PreEscaped(c::STORY_P2.get(lang))) }
                            p { (PreEscaped(c::STORY_P3.get(lang))) }
                        }
                    }
                }

                // PULL 1
                section.pull {
                    div.wrap {
                        blockquote { (PreEscaped(c::PULL1.get(lang))) }
                        cite { "— Hartem Yaakoub" }
                    }
                }

                // SOLUTIONS
                section.section id="solutions" {
                    div.wrap {
                        div.section-num { "N° 02" }
                        div.kicker-2 { (c::SOL_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::SOL_H2.get(lang))) }
                        p.deck { (PreEscaped(c::SOL_DECK.get(lang))) }

                        div.solutions-grid {
                            @for s in c::SOLUTIONS {
                                a class={ "sol " (if s.featured { "sol-feat " } else { "" }) "sol-w" (s.span) } href=(s.url) {
                                    div.hdr {
                                        div.num { (s.num) }
                                        div.tag { (s.badge.get(lang)) }
                                    }
                                    h3 { (s.name) }
                                    p { (s.desc.get(lang)) }
                                    div.ftr {
                                        span { (s.url.trim_start_matches("https://").trim_end_matches("/")) }
                                        span.arr { "→" }
                                    }
                                }
                            }
                        }
                    }
                }

                // TV — Channel 6 spotlight
                section.section id="television" {
                    div.wrap {
                        div.section-num { "N° 03" }
                        div.kicker-2 { (c::TV_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::TV_H2.get(lang))) }

                        div.tv-spotlight {
                            div.q-block {
                                div.src { (c::TV_SOURCE.get(lang)) }
                                blockquote { (PreEscaped(c::TV_QUOTE.get(lang))) }
                                cite {
                                    b { (match lang { Lang::Ar => "حرتام يعقوب", _ => "Hartem Yaakoub" }) }
                                    br;
                                    span { (c::TV_CITE.get(lang)) }
                                }
                                a.watch href="https://youtube.com/shorts/gP3qLsFiqYw" target="_blank" rel="noopener" {
                                    span { (c::TV_WATCH.get(lang)) }
                                    span { " ↗" }
                                }
                            }
                            a.embed href="https://youtube.com/shorts/gP3qLsFiqYw" target="_blank" rel="noopener" aria-label="Watch on Channel 6" {
                                img src="https://img.youtube.com/vi/gP3qLsFiqYw/hqdefault.jpg" alt="Channel 6 Salam Chab — Hartem Yaakoub";
                                span.play aria-hidden="true" {}
                            }
                        }
                    }
                }

                // PULL 2
                section.pull {
                    div.wrap {
                        blockquote { (PreEscaped(c::PULL2.get(lang))) }
                        cite { "— Hartem Yaakoub" }
                    }
                }

                // PRESS WALL (N° 04)
                section.section id="press" {
                    div.wrap {
                        div.section-num { "N° 04" }
                        div.kicker-2 { (c::PRESS_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::PRESS_H2.get(lang))) }
                        p.deck { (c::PRESS_DECK.get(lang)) }

                        div.wall {
                            @for p in c::PRESS_PHOTOS {
                                div class={ "ph " (p.layout) } {
                                    img src={ "/img/" (p.file) } alt=(p.event.get(lang)) loading="lazy";
                                    div.cap {
                                        div.ev { (PreEscaped(p.event.get(lang))) }
                                        div.dt { (p.date) }
                                    }
                                }
                            }
                        }
                    }
                }

                // RECOGNITION + MEDIA LIBRARY (N° 05)
                section.section id="media" {
                    div.wrap {
                        div.section-num { "N° 05" }
                        div.kicker-2 {
                            @match lang {
                                Lang::En => "Recognition & media library",
                                Lang::Fr => "Reconnaissance & médiathèque",
                                Lang::Ar => "التكريم والمكتبة الإعلاميّة",
                            }
                        }
                        h2.head {
                            @match lang {
                                Lang::En => (PreEscaped("The <em>full record.</em>")),
                                Lang::Fr => (PreEscaped("Le <em>dossier complet.</em>")),
                                Lang::Ar => (PreEscaped("<em>السجلّ الكامل.</em>")),
                            }
                        }

                        // Recognition spotlight
                        div.rec-card {
                            a.embed href="https://youtube.com/shorts/vSxWR347G6c" target="_blank" rel="noopener" aria-label="Recognition video" {
                                img src="https://img.youtube.com/vi/vSxWR347G6c/hqdefault.jpg" alt="Honoring Hartem Yaakoub";
                                span.play aria-hidden="true" {}
                            }
                            div {
                                div.label { (c::REC_LABEL.get(lang)) }
                                h3 { (PreEscaped(c::REC_H3.get(lang))) }
                                p { (c::REC_BODY.get(lang)) }
                            }
                        }

                        h3.media-h { (c::MEDIA_LIBRARY_TITLE.get(lang)) }

                        div.media-grid {
                            @for v in c::VIDEOS {
                                a class="vid" href={ (if v.is_short { "https://youtube.com/shorts/" } else { "https://youtu.be/" }) (v.yt_id) } target="_blank" rel="noopener" {
                                    div class={ "vid-thumb" (if v.is_short { " short" } else { "" }) } {
                                        span.badge { (v.badge) }
                                        img src={ "https://img.youtube.com/vi/" (v.yt_id) "/hqdefault.jpg" } alt="" loading="lazy";
                                        span.play aria-hidden="true" {}
                                    }
                                    div.vid-tag { (v.tag.get(lang)) }
                                    div.vid-title { (v.title.get(lang)) }
                                }
                            }
                        }
                    }
                }

                // TIMELINE (N° 06)
                section.section {
                    div.wrap-mid {
                        div.section-num { "N° 06" }
                        div.kicker-2 { (c::TL_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::TL_H2.get(lang))) }

                        div.tl style="margin-top:60px" {
                            @for e in c::TIMELINE {
                                div.tl-item {
                                    div.tl-date { (e.date) }
                                    div.tl-body {
                                        h4 { (PreEscaped(e.title.get(lang))) }
                                        @if let Some(ctx) = &e.ctx {
                                            p { (ctx.get(lang)) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // WRITING (N° 07)
                section.section id="writing" {
                    div.wrap-mid {
                        div.section-num { "N° 07" }
                        div.kicker-2 { (c::WR_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::WR_H2.get(lang))) }

                        div.writes-grid style="margin-top:60px" {
                            a.write href="https://medium.com/@TKAWENTECH/global-innovation-and-entrepreneurial-synergies-a-transformative-experience-at-asep-in-south-korea-e71052047704" target="_blank" rel="noopener" {
                                div.src { (c::WR_SRC.get(lang)) }
                                h3 { (c::WR_TITLE.get(lang)) }
                                p { (c::WR_BODY.get(lang)) }
                                div.read { (c::WR_READ.get(lang)) }
                            }
                        }
                    }
                }

                // CONTACT (N° 08)
                section.contact id="contact" {
                    div.wrap-tight {
                        div.section-num style="justify-content:center" { "N° 08 · " (c::CONTACT_KICKER.get(lang)) }
                        h2.head { (PreEscaped(c::CONTACT_H2.get(lang))) }
                        div.em {
                            a href="mailto:DIRECTION@takawen.dz" { "DIRECTION@takawen.dz" }
                            span.sep { "·" }
                            a href="tel:+213666507935" { "+213 666 507 935" }
                        }
                        div.profiles {
                            a.profile href="https://github.com/hartemyaakoub" { "GitHub" }
                            a.profile href="https://github.com/hartemyaakoub/hartem-tkawen-rs" { "Source ⚡" }
                            a.profile href="https://www.linkedin.com/in/yaakoub-hartem-25a2b722b/" { "LinkedIn" }
                            a.profile href="https://medium.com/@TKAWENTECH" { "Medium" }
                            a.profile href="https://www.tkawen.com/" { "TKAWEN" }
                        }
                    }
                }

                // FOOTER
                footer {
                    div.wrap {
                        div.row {
                            div.col {
                                div.sig {
                                    "Hartem " em { "Yaakoub" }
                                    small { "حرتام يعقوب" }
                                }
                            }
                            div.col {
                                h6 { (c::FOOTER_COMPANY.get(lang)) }
                                p { "Société de Formation et de Services Digitaux — TKAWEN" }
                                p { "R/C 23/00-1880186 A 22" br; "NIF 19623011012910800000" br; "D-U-N-S 35-355-1313" br; "UBMA-STARTUP-2023-00062" }
                            }
                            div.col {
                                h6 { (c::FOOTER_RESOURCES.get(lang)) }
                                a href="/.well-known/security.txt" { "security.txt" }
                                a href="/llms.txt" { "llms.txt" }
                                a href="/schemas/entity.jsonld" { "entity.jsonld" }
                                a href="/sitemap.xml" { "sitemap.xml" }
                                a href="https://github.com/hartemyaakoub/hartem-tkawen-rs" { "source code ⚡" }
                            }
                        }
                        div.colofon {
                            "© 2026 Hartem Yaakoub · "
                            (match lang { Lang::En => "Rendered in", Lang::Fr => "Rendu en", Lang::Ar => "تمّ التوليد في" })
                            " "
                            strong style="color:var(--gold-2)" { (format!("{}μs", render_us)) }
                            " · "
                            a href="https://github.com/hartemyaakoub/hartem-tkawen-rs" target="_blank" rel="noopener" { "Rust · Axum · Maud — AGPL-3.0 ⚡" }
                        }
                    }
                }
            }
        }
    }
}
