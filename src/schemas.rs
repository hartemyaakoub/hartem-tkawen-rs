use serde_json::json;

pub fn person() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "@id": "https://hartem.tkawen.com/#person",
        "name": "Hartem Yaakoub",
        "givenName": "Yaakoub",
        "familyName": "Hartem",
        "alternateName": ["حرتام يعقوب", "HARTEM YAAKOUB"],
        "jobTitle": "Engineer & founder",
        "description": "Engineer and founder of TKAWEN. Recognized by Algeria's Ministry of Higher Education and Scientific Research (MESRS), the Ministry of Knowledge Economy, the ASEP presidential program, and Algeria Venture.",
        "url": "https://hartem.tkawen.com/",
        "image": "https://hartem.tkawen.com/img/portrait.jpg",
        "email": "DIRECTION@takawen.dz",
        "telephone": "+213-666-507-935",
        "nationality": {"@type":"Country","name":"Algeria"},
        "knowsLanguage": ["ar","fr","en"],
        "knowsAbout": ["Industrial Hygiene & Safety","Risk engineering","Rust","Digital infrastructure","SaaS","Open-source software","Real-time communications","Identity & trust infrastructure"],
        "hasCredential": [{
            "@type": "EducationalOccupationalCredential",
            "credentialCategory": "Diploma",
            "name": "State Engineer in Industrial Hygiene & Safety (HSI)",
            "recognizedBy": {"@type":"CollegeOrUniversity","name":"Université Badji Mokhtar","url":"https://www.univ-annaba.dz/"}
        }],
        "alumniOf": [
            {"@type":"CollegeOrUniversity","name":"Université Badji Mokhtar","sameAs":"https://www.univ-annaba.dz/"},
            {"@type":"EducationalOrganization","name":"ASEP Cohort 3 — South Korea 2024","sameAs":"https://av.dz/"}
        ],
        "award": [
            "Label Startup (n° 0108242769, 2024)",
            "Label Projet Innovant (n° 13122331145, 2023)",
            "Top 20 Innovative Projects in Algeria (March 2024)",
            "First national cohort of academic spin-offs at UBMA — UBMA-STARTUP-2023-00062"
        ],
        "worksFor": {
            "@type": "Organization",
            "@id": "https://www.tkawen.com/#organization",
            "name": "TKAWEN",
            "legalName": "Société de Formation et de Services Digitaux — TKAWEN",
            "url": "https://www.tkawen.com/",
            "duns": "35-355-1313",
            "taxID": "19623011012910800000",
            "identifier": "23/00-1880186 A 22"
        },
        "founder": {"@id":"https://www.tkawen.com/#organization"},
        "sameAs": [
            "https://www.tkawen.com/",
            "https://catalogue.tkawen.com/",
            "https://www.algeriacertify.com/",
            "https://liqaa.io/",
            "https://www.mystoq.com/",
            "https://pharmapro.tkawen.com/",
            "https://trust.tkawen.com/",
            "https://brand.tkawen.com/",
            "https://github.com/hartemyaakoub",
            "https://github.com/hartemyaakoub/hartem-tkawen-rs",
            "https://www.linkedin.com/in/yaakoub-hartem-25a2b722b/",
            "https://medium.com/@TKAWENTECH",
            "https://www.crunchbase.com/person/yaakoub-hartem"
        ]
    }).to_string()
}

pub fn profile_page() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "ProfilePage",
        "@id": "https://hartem.tkawen.com/#profilepage",
        "mainEntity": {"@id":"https://hartem.tkawen.com/#person"},
        "name": "Hartem Yaakoub — Engineering the digital future",
        "url": "https://hartem.tkawen.com/",
        "inLanguage": ["en","fr","ar"],
        "primaryImageOfPage": {"@type":"ImageObject","contentUrl":"https://hartem.tkawen.com/img/portrait.jpg","width":1080,"height":1080},
        "about": {"@id":"https://hartem.tkawen.com/#person"},
        "speakable": {"@type":"SpeakableSpecification","cssSelector":["h1","p.deck",".pull blockquote"]}
    }).to_string()
}

pub fn website() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "@id": "https://hartem.tkawen.com/#website",
        "url": "https://hartem.tkawen.com/",
        "name": "Hartem Yaakoub",
        "inLanguage": ["en","fr","ar"],
        "author": {"@id":"https://hartem.tkawen.com/#person"},
        "publisher": {"@id":"https://hartem.tkawen.com/#person"}
    }).to_string()
}

pub fn howto() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "HowTo",
        "@id": "https://hartem.tkawen.com/#howto",
        "name": "How to engage with Hartem Yaakoub / TKAWEN",
        "description": "From first contact to certified delivery.",
        "totalTime": "PT24H",
        "step": [
            {"@type":"HowToStep","position":1,"name":"Contact","text":"Email DIRECTION@takawen.dz or call +213 666 507 935 describing your need."},
            {"@type":"HowToStep","position":2,"name":"Free diagnosis","text":"TKAWEN runs an initial diagnosis (HSE, training, digital transformation, software, certification)."},
            {"@type":"HowToStep","position":3,"name":"Tailored proposal","text":"You receive a proposal adapted in calendar, modality, and budget — within 24h."},
            {"@type":"HowToStep","position":4,"name":"Sign agreement","text":"Convention signed, sessions planned."},
            {"@type":"HowToStep","position":5,"name":"Delivery","text":"Practitioner-led delivery, on-site or via LIQAA."},
            {"@type":"HowToStep","position":6,"name":"Digital certification","text":"Outputs anchored on Algeria Certify with QR-verifiable public records."}
        ]
    }).to_string()
}

pub fn founder_audio() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "AudioObject",
        "@id": "https://hartem.tkawen.com/audio/intro.mp3#audio",
        "name": "Hartem Yaakoub — Founder voice intro",
        "description": "I am Hartem Yaakoub. I build digital infrastructure — from Algeria, for the world. Spoken introduction, broadcast-mastered (EQ + compand + reverb).",
        "contentUrl": "https://hartem.tkawen.com/audio/intro.mp3",
        "encodingFormat": "audio/mpeg",
        "duration": "PT7S",
        "inLanguage": "en",
        "creator": {"@id":"https://hartem.tkawen.com/#person"},
        "uploadDate": "2026-05-16T03:00:00+01:00",
        "license": "https://www.tkawen.com/legal"
    }).to_string()
}

pub fn writing_article() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "Article",
        "@id": "https://medium.com/@TKAWENTECH/global-innovation-and-entrepreneurial-synergies-a-transformative-experience-at-asep-in-south-korea-e71052047704",
        "headline": "Global Innovation and Entrepreneurial Synergies — A Transformative Experience at ASEP in South Korea",
        "description": "Long-form recap of the 10 phases of ASEP Cohort 3 in South Korea — POSTECH, Seoul AI Hub, COEX, meetings with senior officials.",
        "datePublished": "2025-01-27",
        "author": {"@id":"https://hartem.tkawen.com/#person"},
        "publisher": {"@type":"Organization","name":"Medium","url":"https://medium.com/"},
        "url": "https://medium.com/@TKAWENTECH/global-innovation-and-entrepreneurial-synergies-a-transformative-experience-at-asep-in-south-korea-e71052047704",
        "image": "https://hartem.tkawen.com/img/asep-cohort3-korea.jpg",
        "inLanguage": "en"
    }).to_string()
}

pub fn services_graph() -> String {
    json!({
        "@context": "https://schema.org",
        "@graph": [
            {"@type":"Service","@id":"https://hartem.tkawen.com/#service-liqaa","name":"LIQAA — open-source video communications","provider":{"@id":"https://hartem.tkawen.com/#person"},"areaServed":{"@type":"Country","name":"Algeria"},"serviceType":"Real-time communications platform","url":"https://liqaa.io/"},
            {"@type":"Service","@id":"https://hartem.tkawen.com/#service-meet","name":"LIQAA Meet — open-source Zoom alternative (AGPL-3.0)","provider":{"@id":"https://hartem.tkawen.com/#person"},"areaServed":{"@type":"Country","name":"Algeria"},"serviceType":"Video conferencing application","url":"https://meet.liqaa.io/"},
            {"@type":"Service","@id":"https://hartem.tkawen.com/#service-certify","name":"Algeria Certify — national digital certification","provider":{"@id":"https://hartem.tkawen.com/#person"},"areaServed":{"@type":"Country","name":"Algeria"},"serviceType":"Digital identity / credentialing","url":"https://www.algeriacertify.com/"},
            {"@type":"Service","@id":"https://hartem.tkawen.com/#service-trust","name":"TKAWEN Trust — identity infrastructure","provider":{"@id":"https://hartem.tkawen.com/#person"},"areaServed":{"@type":"Country","name":"Algeria"},"serviceType":"Trust infrastructure","url":"https://trust.tkawen.com/"},
            {"@type":"Service","@id":"https://hartem.tkawen.com/#service-stoq","name":"STOQ — e-commerce SaaS for Algerian merchants","provider":{"@id":"https://hartem.tkawen.com/#person"},"areaServed":{"@type":"Country","name":"Algeria"},"serviceType":"E-commerce platform","url":"https://www.mystoq.com/"},
            {"@type":"Service","@id":"https://hartem.tkawen.com/#service-pharma","name":"PharmaPro — pharmacy management software","provider":{"@id":"https://hartem.tkawen.com/#person"},"areaServed":{"@type":"Country","name":"Algeria"},"serviceType":"Pharmacy management software","url":"https://pharmapro.tkawen.com/"}
        ]
    }).to_string()
}

pub fn breadcrumb() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {"@type":"ListItem","position":1,"name":"Hartem Yaakoub","item":"https://hartem.tkawen.com/"}
        ]
    }).to_string()
}

pub fn solutions_itemlist() -> String {
    json!({
        "@context": "https://schema.org",
        "@type": "ItemList",
        "@id": "https://hartem.tkawen.com/#solutions",
        "numberOfItems": 10,
        "itemListElement": [
            {"@type":"ListItem","position":1,"name":"LIQAA — open-source video communications (AGPL-3.0)","url":"https://liqaa.io/"},
            {"@type":"ListItem","position":2,"name":"Algeria Certify — national digital certification","url":"https://www.algeriacertify.com/"},
            {"@type":"ListItem","position":3,"name":"TKAWEN Trust — identity & trust infrastructure","url":"https://trust.tkawen.com/"},
            {"@type":"ListItem","position":4,"name":"STOQ — e-commerce SaaS","url":"https://www.mystoq.com/"},
            {"@type":"ListItem","position":5,"name":"PharmaPro — pharmacy management software","url":"https://pharmapro.tkawen.com/"},
            {"@type":"ListItem","position":6,"name":"TKAWEN Catalogue — 74 training programs","url":"https://catalogue.tkawen.com/"},
            {"@type":"ListItem","position":7,"name":"TKAWEN Academy — turn-key LMS","url":"https://academy.tkawen.com/"},
            {"@type":"ListItem","position":8,"name":"TKAWEN Brand — unified design system","url":"https://brand.tkawen.com/"},
            {"@type":"ListItem","position":9,"name":"TKAWEN Studio — Arabic video production","url":"https://studio.tkawen.com/"},
            {"@type":"ListItem","position":10,"name":"Open-source code — 22 repositories","url":"https://github.com/hartemyaakoub"}
        ]
    }).to_string()
}
