window.onload = function() {
    document.addEventListener('scroll', function(e) {
        localStorage.rr_lastScrollY = window.scrollY;
    });
    var set_promise = browser.runtime.sendMessage({ 
        content : {
            func: "is_sticky",
            url: window.location.href
        }
    }).then(
        setScrollListener,
        function (){
            console.log("Failed to send message to bg handler");
        }
    );

};

function setScrollListener(message) {
    if (message.response) {
        if (localStorage.rr_lastScrollY) {
            window.scrollTo(window.scrollX, parseInt(localStorage.rr_lastScrollY));
        }
    }
}
