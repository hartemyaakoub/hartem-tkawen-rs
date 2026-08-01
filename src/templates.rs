use maud::{html, Markup, PreEscaped, DOCTYPE};
use crate::content as c;
use crate::i18n::Lang;

const CSS: &str = include_str!("../assets/styles.css");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const JS: &str = r#"
(function(){
  // ============== BOOT SEQUENCE (first visit only) ==============
  var boot=document.getElementById('boot');
  if(boot){
    var BOOTED=null;
    try{BOOTED=localStorage.getItem('hartem_booted');}catch(e){}
    if(BOOTED==='v6'){
      boot.remove();
    }else{
      var lines=[
        '> hartem-tkawen-rs · v0.6.0 · cold boot',
        '> tokio runtime spawning ........................ [OK]',
        '> axum 0.7 server initialized ................... [OK]',
        '> maud templates compiled at build-time .......... [OK]',
        '> rust-embed assets loaded (8.8 MB binary) ....... [OK]',
        '> 4 json-ld schemas serialized ................... [OK]',
        '> 11 product mockups embedded .................... [OK]',
        '> trilingual content bundle (en · fr · ar) ....... [OK]',
        '> listening on 127.0.0.1:3088 .................... [OK]',
        '> tls handshake (lets-encrypt) ................... [OK]',
        '> server-timing benchmark ........................ 137μs',
        '',
        '> READY · welcome.'
      ];
      var out=document.getElementById('boot-out');
      var skip=document.getElementById('boot-skip');
      var i=0,timer;
      function next(){
        if(i>=lines.length){
          setTimeout(function(){boot.classList.add('done');setTimeout(function(){boot.remove();},500);},650);
          try{localStorage.setItem('hartem_booted','v6');}catch(e){}
          return;
        }
        var line=document.createElement('div');
        line.className='boot-line';
        line.textContent=lines[i];
        out.appendChild(line);
        i++;
        timer=setTimeout(next,180+Math.random()*80);
      }
      function escape(){
        clearTimeout(timer);
        boot.classList.add('done');
        setTimeout(function(){boot.remove();},300);
        try{localStorage.setItem('hartem_booted','v6');}catch(e){}
      }
      if(skip)skip.addEventListener('click',escape);
      boot.addEventListener('click',function(e){if(e.target===boot)escape();});
      document.addEventListener('keydown',function(e){if(e.key==='Escape')escape();});
      setTimeout(next,400);
    }
  }

  var b=document.body;
  // Mouse spotlight + portrait glitch tracking
  var ps=document.querySelector('.portrait-stack');
  document.addEventListener('mousemove',function(e){
    b.style.setProperty('--mx',e.clientX+'px');
    b.style.setProperty('--my',e.clientY+'px');
    if(ps){
      var r=ps.getBoundingClientRect();
      var cx=r.left+r.width/2,cy=r.top+r.height/2;
      var dx=(e.clientX-cx)/window.innerWidth, dy=(e.clientY-cy)/window.innerHeight;
      // intensity scales with distance from center, max ±5px
      ps.style.setProperty('--rx',(dx*5).toFixed(2)+'px');
      ps.style.setProperty('--ry',(dy*5).toFixed(2)+'px');
    }
  },{passive:true});

  // Scroll progress bar
  var bar=document.querySelector('.progress');
  function tick(){var s=window.scrollY,h=document.documentElement.scrollHeight-window.innerHeight,p=h?s/h:0;bar.style.transform='scaleX('+p+')';}
  window.addEventListener('scroll',tick,{passive:true});tick();

  // Live Annaba clock (Africa/Algiers UTC+1)
  var clk=document.getElementById('local-clock');
  function clock(){
    if(!clk)return;
    try{
      clk.textContent=new Intl.DateTimeFormat('en-GB',{timeZone:'Africa/Algiers',hour:'2-digit',minute:'2-digit',second:'2-digit',hour12:false}).format(new Date());
    }catch(e){
      var d=new Date(Date.now()+3600000);
      clk.textContent=String(d.getUTCHours()).padStart(2,'0')+':'+String(d.getUTCMinutes()).padStart(2,'0')+':'+String(d.getUTCSeconds()).padStart(2,'0');
    }
  }
  clock();setInterval(clock,1000);

  // Live render counter + sparkline of last 20 renders
  var counter=document.getElementById('render-counter');
  var spark=document.querySelector('#sparkline polyline');
  var history=[];
  function pingRender(){
    fetch('/?_='+Date.now(),{method:'HEAD',cache:'no-store'}).then(function(r){
      var st=r.headers.get('server-timing')||'';
      var m=st.match(/dur=([0-9.]+)/);
      if(m){
        var us=Math.round(parseFloat(m[1])*1000);
        if(counter){counter.classList.add('flash');counter.textContent=us;setTimeout(function(){counter.classList.remove('flash');},400);}
        history.push(us);
        if(history.length>20)history.shift();
        if(spark&&history.length>1){
          var mn=Math.min.apply(null,history),mx=Math.max.apply(null,history);
          var rng=Math.max(mx-mn,30);
          var pts=history.map(function(v,i){
            var x=(i/(history.length-1))*80;
            var y=24-((v-mn)/rng)*18-3;
            return x.toFixed(1)+','+y.toFixed(1);
          }).join(' ');
          spark.setAttribute('points',pts);
        }
      }
    }).catch(function(){});
  }
  pingRender();
  setInterval(pingRender,2500);

  // ============== 3D MOUSE TILT on solution cards (skip on touch) ==============
  var TOUCH = !window.matchMedia('(hover: hover)').matches;
  if(!TOUCH){
    document.querySelectorAll('.sol-card').forEach(function(card){
      card.addEventListener('mousemove',function(e){
        var r=card.getBoundingClientRect();
        var x=(e.clientX-r.left)/r.width;
        var y=(e.clientY-r.top)/r.height;
        var rx=(0.5-y)*8;
        var ry=(x-0.5)*8;
        card.style.transform='perspective(1200px) rotateX('+rx.toFixed(2)+'deg) rotateY('+ry.toFixed(2)+'deg) translateY(-6px)';
        card.style.setProperty('--glow-x',(x*100).toFixed(0)+'%');
        card.style.setProperty('--glow-y',(y*100).toFixed(0)+'%');
      });
      card.addEventListener('mouseleave',function(){
        card.style.transform='';
      });
    });
  }

  // Random portrait glitch every 6-12s for 180ms
  if(ps){
    function glitch(){
      ps.classList.add('glitching');
      setTimeout(function(){ps.classList.remove('glitching');},180);
      setTimeout(glitch,6000+Math.random()*6000);
    }
    setTimeout(glitch,3000);
  }

  // ============== TYPEWRITER HERO + AUDIO ==============
  var h1=document.getElementById('hero-h1');
  var soundBtn=document.getElementById('sound-toggle');
  var SOUND=false;
  var actx=null;
  try{SOUND=localStorage.getItem('hartem_sound')==='on';}catch(e){}
  var noiseBuf=null;
  function audioInit(){
    if(!actx){try{actx=new (window.AudioContext||window.webkitAudioContext)();}catch(e){return Promise.reject();}}
    if(!noiseBuf&&actx){
      var sr=actx.sampleRate,len=Math.floor(sr*0.06);
      noiseBuf=actx.createBuffer(1,len,sr);
      var d=noiseBuf.getChannelData(0);
      for(var i=0;i<len;i++)d[i]=Math.random()*2-1;
    }
    if(actx.state==='suspended')return actx.resume();
    return Promise.resolve();
  }
  // Real mechanical keyboard click — IBM Model M aesthetic
  // 4 layers: noise snap, body thump (downward pitch sweep), tactile tick, release noise
  function click(){
    if(!SOUND||!actx||actx.state!=='running'||!noiseBuf)return;
    try{
      var now=actx.currentTime;
      var pitch=0.92+Math.random()*0.16; // ±8% pitch variation per key

      // === LAYER 1: hard plastic snap (high noise burst, bandpass 4-6kHz) ===
      var snap=actx.createBufferSource();
      snap.buffer=noiseBuf;
      snap.playbackRate.value=pitch;
      var snapFilt=actx.createBiquadFilter();
      snapFilt.type='bandpass';
      snapFilt.frequency.value=4800+Math.random()*1500;
      snapFilt.Q.value=1.5;
      var snapGain=actx.createGain();
      snapGain.gain.setValueAtTime(0.45,now);
      snapGain.gain.exponentialRampToValueAtTime(0.001,now+0.014);
      snap.connect(snapFilt);snapFilt.connect(snapGain);snapGain.connect(actx.destination);
      snap.start(now);snap.stop(now+0.02);

      // === LAYER 2: body thump (sine, pitch sweeps down) — the "thock" ===
      var thump=actx.createOscillator(),thumpGain=actx.createGain();
      thump.type='sine';
      thump.frequency.setValueAtTime(220*pitch,now);
      thump.frequency.exponentialRampToValueAtTime(45,now+0.06);
      thumpGain.gain.setValueAtTime(0.32,now);
      thumpGain.gain.exponentialRampToValueAtTime(0.0008,now+0.085);
      thump.connect(thumpGain);thumpGain.connect(actx.destination);
      thump.start(now);thump.stop(now+0.1);

      // === LAYER 3: tactile mid-click (square, sharp attack) ===
      var tact=actx.createOscillator(),tactGain=actx.createGain();
      tact.type='square';
      tact.frequency.value=1600+Math.random()*300;
      tactGain.gain.setValueAtTime(0.08,now);
      tactGain.gain.exponentialRampToValueAtTime(0.0005,now+0.012);
      tact.connect(tactGain);tactGain.connect(actx.destination);
      tact.start(now);tact.stop(now+0.018);

      // === LAYER 4: release whisper (subtle high noise at 8ms delay) ===
      var rel=actx.createBufferSource();
      rel.buffer=noiseBuf;
      rel.playbackRate.value=pitch*1.2;
      var relFilt=actx.createBiquadFilter();
      relFilt.type='highpass';
      relFilt.frequency.value=6500;
      var relGain=actx.createGain();
      relGain.gain.setValueAtTime(0.12,now+0.008);
      relGain.gain.exponentialRampToValueAtTime(0.0005,now+0.022);
      rel.connect(relFilt);relFilt.connect(relGain);relGain.connect(actx.destination);
      rel.start(now+0.008);rel.stop(now+0.025);
    }catch(e){}
  }
  function setSoundLabel(){if(soundBtn)soundBtn.textContent='[ SOUND '+(SOUND?'ON':'OFF')+' ]';}
  if(soundBtn){
    setSoundLabel();
    soundBtn.addEventListener('click',function(){
      SOUND=!SOUND;
      try{localStorage.setItem('hartem_sound',SOUND?'on':'off');}catch(e){}
      setSoundLabel();
      if(SOUND){
        audioInit().then(function(){
          // Immediate feedback click
          click();
          // Re-run typewriter with sound
          if(h1)typeH1();
        }).catch(function(){});
      }
    });
  }

  function typeH1(){
    if(!h1)return;
    var raw=h1.getAttribute('data-text');
    if(!raw)return;
    // parse to walk text + em nodes
    var tmp=document.createElement('div');
    tmp.innerHTML=raw;
    var nodes=[];
    function walk(parent,wrapTag){
      parent.childNodes.forEach(function(n){
        if(n.nodeType===3){
          // text node — push each char
          var t=n.textContent;
          for(var i=0;i<t.length;i++)nodes.push({c:t[i],tag:wrapTag});
        }else if(n.nodeType===1){
          walk(n,n.tagName.toLowerCase());
        }
      });
    }
    walk(tmp,null);
    h1.innerHTML='<span class="caret">_</span>';
    var caret=h1.querySelector('.caret');
    var i=0,currentEm=null;
    function tick(){
      if(i>=nodes.length){if(caret)caret.remove();return;}
      var n=nodes[i];
      if(n.tag==='em'){
        if(!currentEm){currentEm=document.createElement('em');h1.insertBefore(currentEm,caret);}
        currentEm.appendChild(document.createTextNode(n.c));
      }else{
        currentEm=null;
        h1.insertBefore(document.createTextNode(n.c),caret);
      }
      if(n.c.trim())click();
      i++;
      setTimeout(tick,38+Math.random()*22);
    }
    setTimeout(tick,100);
  }
  if(h1){
    // hide instantly to avoid flash
    h1.style.minHeight=h1.offsetHeight+'px';
    typeH1();
  }

  // (FOUNDER VOICE PLAYER handled by inline onclick on the button itself)

  // ============== MATRIX CODE RAIN (idle 5s) ==============
  var canvas=document.getElementById('rain');
  if(canvas && ps){
    var ctx=canvas.getContext('2d');
    var rainOn=false, raf=null, lastMove=Date.now();
    var WORDS=['HARTEM','YAAKOUB','TKAWEN','LIQAA','MEET','CERTIFY','PHARMA','STOQ','TRUST','BRAND','STUDIO','TRACK','CATALOGUE','ANNABA','ALGERIA','HSI','UBMA','ASEP','RUST','AXUM','MAUD'];
    var SYMS='{}[]<>/=>;:.0123456789';
    var cols=[],fontSize=14;
    function setup(){
      var dpr=Math.min(window.devicePixelRatio||1,2);
      var w=canvas.offsetWidth,h=canvas.offsetHeight;
      canvas.width=w*dpr;canvas.height=h*dpr;
      ctx.scale(dpr,dpr);
      ctx.font=fontSize+'px JetBrains Mono, monospace';
      var n=Math.floor(w/fontSize);
      cols=[];for(var i=0;i<n;i++)cols.push({y:Math.random()*-h,word:'',ci:0});
    }
    function draw(){
      var w=canvas.offsetWidth,h=canvas.offsetHeight;
      ctx.fillStyle='rgba(0,0,0,0.10)';
      ctx.fillRect(0,0,w,h);
      ctx.fillStyle='rgba(212,175,55,0.85)';
      for(var i=0;i<cols.length;i++){
        var c=cols[i];
        if(!c.word||c.ci>=c.word.length){
          c.word=Math.random()<0.55?WORDS[Math.floor(Math.random()*WORDS.length)]:SYMS[Math.floor(Math.random()*SYMS.length)];
          c.ci=0;
        }
        var ch=c.word[c.ci]||'·';
        ctx.fillStyle=c.y<24?'rgba(244,217,122,1)':'rgba(212,175,55,0.7)';
        ctx.fillText(ch,i*fontSize,c.y);
        c.y+=fontSize;c.ci++;
        if(c.y>h+30)c.y=Math.random()*-100;
      }
      if(rainOn)raf=requestAnimationFrame(draw);
    }
    function start(){if(rainOn)return;rainOn=true;canvas.style.opacity='1';draw();}
    function stop(){rainOn=false;canvas.style.opacity='0';if(raf)cancelAnimationFrame(raf);}
    setup();
    window.addEventListener('resize',function(){setup();},{passive:true});
    document.addEventListener('mousemove',function(){lastMove=Date.now();stop();},{passive:true});
    setInterval(function(){if(Date.now()-lastMove>5000 && !rainOn)start();},800);
  }

  // ============== LIVE INFRASTRUCTURE STATUS ==============
  var statusGrid=document.getElementById('infra-status');
  if(statusGrid){
    var HOSTS=[
      {h:'liqaa.io',name:'LIQAA'},
      {h:'meet.liqaa.io',name:'LIQAA Meet'},
      {h:'www.algeriacertify.com',name:'Algeria Certify'},
      {h:'trust.tkawen.com',name:'TKAWEN Trust'},
      {h:'www.mystoq.com',name:'STOQ'},
      {h:'pharmapro.tkawen.com',name:'PharmaPro'},
      {h:'catalogue.tkawen.com',name:'Catalogue'},
      {h:'www.tkawen.com',name:'TKAWEN'},
      {h:'brand.tkawen.com',name:'Brand'},
      {h:'studio.tkawen.com',name:'Studio'}
    ];
    statusGrid.innerHTML=HOSTS.map(function(s,i){
      return '<a class="ist" target="_blank" rel="noopener" href="https://'+s.h+'" data-i="'+i+'">'+
        '<span class="ist-dot pending"></span>'+
        '<span class="ist-name">'+s.name+'</span>'+
        '<span class="ist-ms" data-ms></span>'+
        '</a>';
    }).join('');
    function ping(host,cb){
      var img=new Image();
      var start=performance.now();
      var done=false;
      img.onload=function(){if(done)return;done=true;cb(true,Math.round(performance.now()-start));};
      img.onerror=function(){if(done)return;done=true;cb(true,Math.round(performance.now()-start));};
      img.src='https://'+host+'/favicon.ico?_='+Date.now();
      setTimeout(function(){if(done)return;done=true;cb(false,0);},5000);
    }
    function update(){
      HOSTS.forEach(function(s,i){
        ping(s.h,function(up,ms){
          var el=statusGrid.querySelector('.ist[data-i="'+i+'"]');
          if(!el)return;
          var dot=el.querySelector('.ist-dot');
          var msEl=el.querySelector('.ist-ms');
          dot.classList.remove('pending');
          dot.classList.add(up?'up':'down');
          if(msEl)msEl.textContent=up?(ms+'ms'):'down';
        });
      });
    }
    update();
    setInterval(update,30000);
  }
})();
if('serviceWorker' in navigator){navigator.serviceWorker.register('/sw.js').catch(function(){});}
"#;

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
    let n_platforms = match lang { Lang::En=>"platforms", Lang::Fr=>"plateformes", Lang::Ar=>"منصّة" };
    let n_langs = match lang { Lang::En=>"languages", Lang::Fr=>"langues", Lang::Ar=>"لغات" };
    let n_repos = match lang { Lang::En=>"open repos", Lang::Fr=>"dépôts ouverts", Lang::Ar=>"مستودع مفتوح" };

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
                meta name="theme-color" content="#000000";
                link rel="icon" href="/favicon.svg";
                link rel="apple-touch-icon" href="/img/portrait.jpg";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Fraunces:ital,opsz,wght@0,9..144,300;0,9..144,400;0,9..144,500;1,9..144,300;1,9..144,400&family=Inter:wght@300;400;500;600&family=Cairo:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap";
                style { (PreEscaped(CSS)) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::person())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::profile_page())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::website())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::solutions_itemlist())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::howto())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::founder_audio())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::writing_article())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::services_graph())) }
                script type="application/ld+json" { (PreEscaped(crate::schemas::breadcrumb())) }
            }
            body lang=(lang.code()) {

                // ===== BOOT SEQUENCE OVERLAY =====
                div #"boot" {
                    div.boot-wrap {
                        div.boot-head {
                            span.boot-cursor { "█" }
                            span { "hartem-tkawen-rs · cold boot · " (VERSION) }
                        }
                        div #"boot-out" {}
                        button #"boot-skip" { "skip ▸" }
                    }
                }

                div.progress {}

                // MASTHEAD — minimal floating mix-blend with live sparkline
                header.masthead {
                    span.issue { (c::ISSUE.get(lang)) }
                    a.mark href="/" { "Hartem " em { "Yaakoub" } }
                    div.sparkline-wrap {
                        svg #"sparkline" viewBox="0 0 80 24" preserveAspectRatio="none" {
                            polyline points="0,12" stroke="currentColor" fill="none" "stroke-width"="1.2" "stroke-linejoin"="round" "stroke-linecap"="round" {}
                        }
                        span.spark-label { "render · live" }
                    }
                    button #"sound-toggle" type="button" "aria-label"="Toggle typewriter sound" { "[ SOUND OFF ]" }
                    div.lang {
                        a href="/?lang=en" class=(if matches!(lang, Lang::En) { "cur" } else { "" }) { "EN" }
                        a href="/?lang=fr" class=(if matches!(lang, Lang::Fr) { "cur" } else { "" }) { "FR" }
                        a href="/?lang=ar" class=(if matches!(lang, Lang::Ar) { "cur" } else { "" }) { "AR" }
                    }
                }

                // HERO — cinematic split, mouse-spotlight
                section.hero {
                    div.left {
                        div.kicker { (c::HERO_KICKER.get(lang)) }
                        h1 #"hero-h1" data-text=(c::HERO_H1.get(lang)) { (PreEscaped(c::HERO_H1.get(lang))) }
                        p.deck { (PreEscaped(c::HERO_DECK.get(lang))) }
                        button #"hear-founder" type="button" onclick="(function(b){if(b._a){b._a.pause();b._a.currentTime=0;b._a=null;b.classList.remove('playing');var l=b.querySelector('.hf-label');if(l)l.textContent='HEAR FOUNDER';var t=b.querySelector('.hf-time');if(t)t.textContent='0:07';return;}var a=new Audio('/audio/intro.mp3?_='+Date.now());b._a=a;b.classList.add('playing');a.addEventListener('ended',function(){b.classList.remove('playing');var l=b.querySelector('.hf-label');if(l)l.textContent='HEAR FOUNDER';var t=b.querySelector('.hf-time');if(t)t.textContent='0:07';b._a=null;});a.addEventListener('timeupdate',function(){var el=b.querySelector('.hf-time');if(el&&a.duration){el.textContent='0:'+String(Math.max(0,Math.ceil(a.duration-a.currentTime))).padStart(2,'0');}});var p=a.play();if(p&&p.catch){p.catch(function(e){alert('Audio playback failed: '+e.name+' — '+e.message);b.classList.remove('playing');b._a=null;});}})(this);" "aria-label"="Hear founder voice intro" {
                            span.hf-icon { "▶" }
                            span.hf-label { "HEAR FOUNDER" }
                            span.hf-waves { span.w {} span.w {} span.w {} span.w {} span.w {} span.w {} span.w {} span.w {} }
                            span.hf-time { "0:07" }
                        }
                        // (audio created on-demand via inline onclick)
                        div.meta {
                            span { b { "11" } " " (n_platforms) }
                            span.sep { "·" }
                            span { b { "3" } " " (n_langs) }
                            span.sep { "·" }
                            span { b { "22" } " " (n_repos) }
                            span.sep { "·" }
                            span { b { "ASEP" } " · 🇰🇷" }
                            span.sep { "·" }
                            span { b { "D-U-N-S" } " 35-355-1313" }
                        }
                    }
                    div.right {
                        div.console {
                            // Frame corner brackets
                            span.cor.tl {}
                            span.cor.tr {}
                            span.cor.bl {}
                            span.cor.br {}

                            // Console title bar
                            div.console-bar {
                                span.dot {} span.dot {} span.dot {}
                                span.console-title { "HARTEM_YAAKOUB.PORTRAIT.v3" }
                                span.console-mode { "[LIVE]" }
                            }

                            // Layered portrait: base + 3 RGB layers + halftone + scan + code rain
                            div.portrait-stack {
                                canvas #"rain" {}
                                img.layer.base src="/img/portrait.jpg" alt="Hartem Yaakoub";
                                img.layer.r src="/img/portrait.jpg" alt="" aria-hidden="true";
                                img.layer.g src="/img/portrait.jpg" alt="" aria-hidden="true";
                                img.layer.b src="/img/portrait.jpg" alt="" aria-hidden="true";
                                div.halftone {}
                                div.scanlines {}
                                div.frame-ticks {}
                            }

                            // HUD stats sidebar
                            div.hud {
                                div.hud-row { span.k { "subject" } span.v { "Hartem Yaakoub · solo engineer · not an institution" } }
                                div.hud-row { span.k { "discipline" } span.v { "HSI · Industrial Hygiene & Safety" } }
                                div.hud-row { span.k { "alma_mater" } span.v { "Université Badji Mokhtar · Annaba" } }
                                div.hud-row { span.k { "company" } span.v { "TKAWEN · UBMA-STARTUP-2023-00062" } }
                                div.hud-row { span.k { "base" } span.v { "Annaba · 🇩🇿 Algeria" } }
                                div.hud-row { span.k { "languages" } span.v { "ar · fr · en" } }
                                div.hud-row { span.k { "status" } span.v.live { span.pulse-dot {} "ACTIVE · " span #"local-clock" { "—" } } }
                                div.hud-row { span.k { "render" } span.v { span #"render-counter" { "138" } "μs · Rust binary" } }
                                div.hud-row { span.k { "build" } span.v { "hartem-rs/" (VERSION) } }
                            }
                        }
                    }
                }

                // MARQUEE — auto-scroll partner logos
                section.marquee {
                    div.marquee-label { (c::ENDORSE_LABEL.get(lang)) }
                    div.marquee-track {
                        @for _ in 0..2 {
                            @for p in c::PARTNERS {
                                img src={ "/img/partners/" (p.file) } alt=(p.alt) title=(p.title);
                            }
                        }
                    }
                }

                // INFRASTRUCTURE STATUS — live ping of all 11 products
                section.infra {
                    div.infra-head {
                        span.live-pulse {}
                        span {
                            @match lang {
                                Lang::En => "Live infrastructure · pinging from your browser right now",
                                Lang::Fr => "Infrastructure live · ping depuis votre navigateur en ce moment",
                                Lang::Ar => "البنية التحتيّة الحيّة · يُختبر من متصفّحك الآن",
                            }
                        }
                    }
                    div #"infra-status" {}
                }

                // STATS — giant numbers
                section.stats {
                    div.stats-label {
                        @match lang {
                            Lang::En => "By the numbers",
                            Lang::Fr => "En chiffres",
                            Lang::Ar => "بالأرقام",
                        }
                    }
                    div.stats-grid {
                        div.stat {
                            div.n { "4,193" sup { "+" } }
                            div.l {
                                @match lang { Lang::En=>b{"Certificates issued"}, Lang::Fr=>b{"Certificats émis"}, Lang::Ar=>b{"شهادة مُصدَرة"} }
                                @match lang { Lang::En=>"Tamper-proof · QR-verifiable · Algeria Certify.", Lang::Fr=>"Inviolables · vérifiables QR · Algeria Certify.", Lang::Ar=>"محميّة · قابلة للتحقّق · Algeria Certify." }
                            }
                        }
                        div.stat {
                            div.n { "4,132" sup { "+" } }
                            div.l {
                                @match lang { Lang::En=>b{"Institutions verified"}, Lang::Fr=>b{"Institutions vérifiées"}, Lang::Ar=>b{"مؤسسة موثَّقة"} }
                                @match lang { Lang::En=>"On Algeria Certify · public 24/7 verification.", Lang::Fr=>"Sur Algeria Certify · vérification publique 24/7.", Lang::Ar=>"على Algeria Certify · تحقّق علني 24/7." }
                            }
                        }
                        div.stat {
                            div.n { "58" sup { "/58" } }
                            div.l {
                                @match lang { Lang::En=>b{"Wilayas covered"}, Lang::Fr=>b{"Wilayas couvertes"}, Lang::Ar=>b{"ولاية مغطّاة"} }
                                @match lang { Lang::En=>"Full national territory.", Lang::Fr=>"Territoire national complet.", Lang::Ar=>"كامل التراب الوطني." }
                            }
                        }
                        div.stat {
                            div.n { span #"render-counter" { "138" } sup { "μs" } }
                            div.l {
                                @match lang { Lang::En=>b{"Per-request render"}, Lang::Fr=>b{"Rendu par requête"}, Lang::Ar=>b{"تَوليد لكلّ طلب"} }
                                @match lang { Lang::En=>"This page is a Rust binary.", Lang::Fr=>"Cette page est un binaire Rust.", Lang::Ar=>"هذه الصفحة Rust binary." }
                            }
                        }
                    }
                }

                // STORY (N° 01)
                section.cine id="story" {
                    div.num { "N° 01" }
                    div.eyebrow { (c::STORY_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::STORY_H2.get(lang))) }
                    p.sub { (PreEscaped(c::STORY_DECK.get(lang))) }

                    div.editorial {
                        p { (PreEscaped(c::STORY_P1.get(lang))) }
                        p { (PreEscaped(c::STORY_P2.get(lang))) }
                        p { (PreEscaped(c::STORY_P3.get(lang))) }
                    }
                }

                // PULL 1 — cinematic
                section.pull {
                    blockquote { (PreEscaped(c::PULL1.get(lang))) }
                    cite { "— Hartem Yaakoub" }
                }

                // SOLUTIONS (N° 02) — feature list rows
                section.cine id="solutions" {
                    div.num { "N° 02" }
                    div.eyebrow { (c::SOL_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::SOL_H2.get(lang))) }
                    p.sub { (PreEscaped(c::SOL_DECK.get(lang))) }

                    div.sols {
                        @for s in c::SOLUTIONS {
                            a class={ "sol-card " (if s.featured { "feat" } else { "" }) } href=(s.url) {
                                div.sol-shot {
                                    img src={ "/img/products/" (s.shot) } alt={ (s.name) " — screenshot" } loading="lazy";
                                    div.sol-shot-overlay {
                                        span.sol-host { (s.url.trim_start_matches("https://").trim_end_matches("/")) }
                                    }
                                }
                                div.sol-info {
                                    div.sol-head {
                                        span.sol-num { (s.num) }
                                        span.sol-tag { (s.badge.get(lang)) }
                                    }
                                    h3 { (s.name) }
                                    p.sol-desc { (s.desc.get(lang)) }
                                    div.sol-metric {
                                        span.sol-metric-icon { "▲" }
                                        span { (s.metric.get(lang)) }
                                    }
                                    div.sol-cta {
                                        span { (s.url.trim_start_matches("https://").trim_end_matches("/")) }
                                        span.arr { "→" }
                                    }
                                }
                            }
                        }
                    }
                }

                // TV — Channel 6 (N° 03)
                section.tv id="television" {
                    div.num { "N° 03" }
                    div.eyebrow { (c::TV_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::TV_H2.get(lang))) }

                    div.tv-grid {
                        div {
                            div.tv-src {
                                span.dot {}
                                span { (c::TV_SOURCE.get(lang)) }
                            }
                            blockquote { (PreEscaped(c::TV_QUOTE.get(lang))) }
                            cite {
                                b { (match lang { Lang::Ar => "حرتام يعقوب", _ => "Hartem Yaakoub" }) }
                                br;
                                span { (c::TV_CITE.get(lang)) }
                            }
                            br;
                            a.tv-watch href="https://youtube.com/shorts/gP3qLsFiqYw" target="_blank" rel="noopener" {
                                span { (c::TV_WATCH.get(lang)) }
                                span { " ↗" }
                            }
                        }
                        a.tv-vid href="https://youtube.com/shorts/gP3qLsFiqYw" target="_blank" rel="noopener" aria-label="Watch on Channel 6" {
                            img src="https://img.youtube.com/vi/gP3qLsFiqYw/hqdefault.jpg" alt="Channel 6";
                            span.play aria-hidden="true" {}
                        }
                    }
                }

                // PRESS WALL (N° 04)
                section.cine id="press" {
                    div.num { "N° 04" }
                    div.eyebrow { (c::PRESS_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::PRESS_H2.get(lang))) }
                    p.sub { (c::PRESS_DECK.get(lang)) }

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

                // RECOGNITION + MEDIA (N° 05)
                section.cine id="media" {
                    div.num { "N° 05" }
                    div.eyebrow {
                        @match lang {
                            Lang::En => "Recognition & media library",
                            Lang::Fr => "Reconnaissance & médiathèque",
                            Lang::Ar => "التكريم والمكتبة الإعلاميّة",
                        }
                    }
                    h2 {
                        @match lang {
                            Lang::En => (PreEscaped("The <em>full record.</em>")),
                            Lang::Fr => (PreEscaped("Le <em>dossier complet.</em>")),
                            Lang::Ar => (PreEscaped("<em>السجلّ الكامل.</em>")),
                        }
                    }

                    div.rec {
                        a.rec-vid href="https://youtube.com/shorts/vSxWR347G6c" target="_blank" rel="noopener" aria-label="Recognition video" {
                            img src="https://img.youtube.com/vi/vSxWR347G6c/hqdefault.jpg" alt="Honoring Hartem Yaakoub";
                            span.play aria-hidden="true" {}
                        }
                        div.rec-body {
                            div.lab { (c::REC_LABEL.get(lang)) }
                            h3 { (PreEscaped(c::REC_H3.get(lang))) }
                            p { (c::REC_BODY.get(lang)) }
                        }
                    }

                    div.media-h { (c::MEDIA_LIBRARY_TITLE.get(lang)) }

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

                // PULL 2
                section.pull {
                    blockquote { (PreEscaped(c::PULL2.get(lang))) }
                    cite { "— Hartem Yaakoub" }
                }

                // TIMELINE (N° 06)
                section.cine {
                    div.num { "N° 06" }
                    div.eyebrow { (c::TL_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::TL_H2.get(lang))) }

                    div.tl {
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

                // WRITING (N° 07)
                section.cine id="writing" {
                    div.num { "N° 07" }
                    div.eyebrow { (c::WR_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::WR_H2.get(lang))) }

                    div.writes {
                        a.write href="https://medium.com/@TKAWENTECH/global-innovation-and-entrepreneurial-synergies-a-transformative-experience-at-asep-in-south-korea-e71052047704" target="_blank" rel="noopener" {
                            div.src { (c::WR_SRC.get(lang)) }
                            h3 { (c::WR_TITLE.get(lang)) }
                            p { (c::WR_BODY.get(lang)) }
                            div.read { (c::WR_READ.get(lang)) }
                        }
                    }
                }

                // DOWNLOAD THE BINARY (N° 08) — proof of open source
                section.cine id="download" {
                    div.num { "N° 08" }
                    div.eyebrow {
                        @match lang {
                            Lang::En => "Download the binary",
                            Lang::Fr => "Télécharger le binaire",
                            Lang::Ar => "حمّل الـbinary",
                        }
                    }
                    h2 {
                        @match lang {
                            Lang::En => (PreEscaped("Run this exact site <em>on your own machine.</em>")),
                            Lang::Fr => (PreEscaped("Exécutez ce site <em>sur votre propre machine.</em>")),
                            Lang::Ar => (PreEscaped("شغّل هذا الموقع <em>على جهازك أنت.</em>")),
                        }
                    }
                    p.sub {
                        @match lang {
                            Lang::En => "The 8.8 MB Rust binary that serves this page — embedded fonts, embedded images, embedded JSON-LD schemas, embedded everything. Download it. Run it. Inspect it. The source is on GitHub under AGPL-3.0.",
                            Lang::Fr => "Le binaire Rust de 8,8 Mo qui sert cette page — polices intégrées, images intégrées, schémas JSON-LD intégrés, tout intégré. Téléchargez-le. Exécutez-le. Inspectez-le. Source sur GitHub sous AGPL-3.0.",
                            Lang::Ar => "الـRust binary بحجم 8.8 ميغا الذي يَخدم هذه الصفحة — خطوط مدمجة، صور مدمجة، schemas JSON-LD مدمجة، كلّ شيء مدمج. حمّله. شغّله. افحصه. المصدر على GitHub بـAGPL-3.0.",
                        }
                    }
                    div.dl-row {
                        a.dl-btn href="/download/hartem-tkawen-rs" download="hartem-tkawen-rs" {
                            span.dl-icon { "⬇" }
                            span.dl-text {
                                span.dl-title {
                                    @match lang {
                                        Lang::En => "Linux x86-64 binary",
                                        Lang::Fr => "Binaire Linux x86-64",
                                        Lang::Ar => "Binary Linux x86-64",
                                    }
                                }
                                span.dl-meta { "hartem-tkawen-rs · v" (VERSION) " · ~8.8 MB · AGPL-3.0" }
                            }
                        }
                        a.dl-btn.alt href="https://github.com/hartemyaakoub/hartem-tkawen-rs" target="_blank" rel="noopener" {
                            span.dl-icon { "{ }" }
                            span.dl-text {
                                span.dl-title {
                                    @match lang {
                                        Lang::En => "View source on GitHub",
                                        Lang::Fr => "Voir la source sur GitHub",
                                        Lang::Ar => "اطّلع على المصدر",
                                    }
                                }
                                span.dl-meta { "@hartemyaakoub/hartem-tkawen-rs" }
                            }
                        }
                    }
                }

                // CONTACT (N° 09) — cinematic ending
                section.contact id="contact" {
                    div.num { "N° 09 · " (c::CONTACT_KICKER.get(lang)) }
                    h2 { (PreEscaped(c::CONTACT_H2.get(lang))) }
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

                // FOOTER — signature + Rust badge
                footer {
                    div.sig {
                        "Hartem " em { "Yaakoub" }
                        small { "حرتام يعقوب" }
                    }
                    div.colofon {
                        "TKAWEN · R/C 23/00-1880186 A 22 · D-U-N-S 35-355-1313 · UBMA-STARTUP-2023-00062"
                        br;
                        a href="/.well-known/security.txt" { "security.txt" } " · "
                        a href="/llms.txt" { "llms.txt" } " · "
                        a href="/schemas/entity.jsonld" { "entity" } " · "
                        a href="/sitemap.xml" { "sitemap" }
                        br;
                        div.badge {
                            (match lang { Lang::En => "Rendered in", Lang::Fr => "Rendu en", Lang::Ar => "تمّ التوليد في" })
                            " "
                            strong { (format!("{}μs", render_us)) }
                            " · "
                            a href="https://github.com/hartemyaakoub/hartem-tkawen-rs" target="_blank" rel="noopener" { "Rust · Axum · Maud — AGPL-3.0 ⚡" }
                        }
                    }
                }

                script { (PreEscaped(JS)) }
            }
        }
    }
}
