// Mobile sidebar toggle
function toggleMobileSidebar(btn) {
  var expanded = btn.getAttribute('aria-expanded') === 'true';
  var menuId = btn.getAttribute('aria-controls');
  var menu = document.getElementById(menuId);
  if (!menu) return;

  btn.setAttribute('aria-expanded', String(!expanded));
  menu.classList.toggle('hidden');
}

// Copy install command
function copyCommand(cmd, btn) {
  function showCopied() {
    btn.innerHTML = '<svg class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg><span class="copy-tooltip">Copied!</span>';
    setTimeout(function() {
      btn.innerHTML = '<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/></svg>';
    }, 2000);
  }

  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(cmd).then(showCopied).catch(function() {
      showCopied();
    });
  }
}

// Mobile nav toggle
var mobileBtn = document.getElementById('mobile-menu-btn');
var mobileMenu = document.getElementById('mobile-menu');
var iconOpen = document.getElementById('menu-icon-open');
var iconClose = document.getElementById('menu-icon-close');

if (mobileBtn) {
  mobileBtn.addEventListener('click', function() {
    var isOpen = !mobileMenu.classList.contains('hidden');
    mobileMenu.classList.toggle('hidden');
    iconOpen.classList.toggle('hidden');
    iconClose.classList.toggle('hidden');
    mobileBtn.setAttribute('aria-expanded', String(!isOpen));
  });
}

// Fetch GitHub star count
fetch('https://api.github.com/repos/Brake-Labs/settl')
  .then(function(res) { return res.json(); })
  .then(function(data) {
    var count = data.stargazers_count;
    if (count !== undefined) {
      var el = document.getElementById('star-count');
      if (el) el.textContent = count >= 1000 ? (count / 1000).toFixed(1) + 'k' : count;
    }
  })
  .catch(function() {});

// Scroll-triggered animations
document.addEventListener('DOMContentLoaded', function() {
  var observer = new IntersectionObserver(function(entries) {
    entries.forEach(function(entry) {
      if (entry.isIntersecting) {
        entry.target.classList.add('is-visible');
        observer.unobserve(entry.target);
      }
    });
  }, { threshold: 0.1 });

  document.querySelectorAll('.animate-on-scroll').forEach(function(el) {
    observer.observe(el);
  });
});
