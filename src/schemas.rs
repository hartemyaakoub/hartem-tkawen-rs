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
