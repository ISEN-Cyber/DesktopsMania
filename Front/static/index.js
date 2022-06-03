
/* define constant */
const TYPE_INFO = {
    ERROR: "error",
    VALID: "validation"
}

const LEVEL = {
    NONE: "NONE",
    USER: "USER",
    ADMIN: "ADMIN",
}

const METHOD = {
    POST: "POST",
    GET: "GET",
}

/**
 * Print an information to the user
 * @param {string} title 
 * @param {string} message 
 * @param {string|null} type 
 */
function viewInformation(title = "", message = "", type = null) {
    function doThat() {
        document.body.dataset.alert = type
        let info = document.getElementById("info")
        info.querySelector("[data-title]").firstChild.data = title
        info.querySelector("[data-message]").firstChild.data = message
    }
    doOnCharge(doThat)
}

const LOG_ERROR = {
    FUNCTION_NOT_SET: () => { console.log("function not set") },
    BAD_ARGUMENT: () => { console.log("Bad argument") },
    BAD_MOVE: () => { console.log("Bad move") },
}
/* default user level */
let user = "NONE"
let constantPath = ""

/**
 * Execute the callback function after the windows loading
 * @param {function} callBack 
 * @returns any
 */
function doOnCharge(callBack) {

    if (document.body) {

        return callBack()
    }
    else
        window.addEventListener("load", (e) => {
            return callBack()
        })
}

/**
 * The default traitement json function
 * @param {string} e 
 */
function defaultCall(e) {
    let jsonFile = JSON.parse(e)
    if (jsonFile) {
        if (jsonFile.code && jsonFile.message && jsonFile.extra) {
            viewInformation(jsonFile.message, jsonFile.extra, TYPE_INFO.VALID)
        }
        else {

        }
    }
}

window.onhashchange = (e) => {
    getUrl(document.body.querySelector(location.hash))
}

/**
 * Get and push all data url in child of an element
 * @param {object} element a dom element
 */
function getUrl(element) {
    element.querySelectorAll("[data-geturl]").forEach(
        (item) => {
            let arg = new FormData()
            let method = METHOD.GET
            let send = true
            for (let encore in item.dataset) {
                switch (encore) {
                    case "seturl": break;
                    case "geturl": break;
                    default:
                        console.log(encore)
                        arg.append(encore, item.dataset[encore])
                        method = METHOD.POST
                        break;
                }
            }
            if (send)
            doHttpRequest(item.dataset.geturl,arg , (text) => {
                let jsonFile = JSON.parse(text)
                if (jsonFile) {
                    pushElement(item, jsonFile)
                    setUrl(item)
                    for (form of item.querySelectorAll("form"))
                        setForm(form)
                }
            }, (e) => { viewInformation("tipical error", e, TYPE_INFO.ERROR) }, method)
        }
    )
}

/**
 * push data in item with dataset
 * @param {objet} item - a dom element
 * @param {json} data - a format dictionnary 
 */
function pushElement(item, data) {
    if (Array.isArray(data)) {
        item.querySelectorAll("[data-copy=\"true\"]").forEach((e) => { e.remove() })
        data.forEach(
            (inside) => {
                let element = item.querySelector("[data-copy=\"false\"]")
                if (element) {
                    
                    let news = element.cloneNode(true)
                    news.dataset.copy = "true"
                    pushElement(news, inside)
                    item.appendChild(news)
                    console.log(item.parentNode)
                }
                else {
                    console.log("heuuu... please il manque un data-copy la")
                }
            }
        )
    }
    else {
        if (typeof data === "string")
        item.querySelector("[data-string]").appendChild(document.createTextNode(data))
        else
        for (let name in data) {
            let element = item.querySelectorAll("[data-" + name + "]")
            for (let small of element)
                if (small.dataset.seturl)
                    small.dataset[name] = data[name]
                else if (small.dataset.geturlafter)
                {
                    let arg = new FormData()
                    let method = METHOD.GET
      
                    for (let encore in small.dataset) {
                        switch (encore) {
                            case "geturlafter": break;
                            default:
                                console.log(encore)
                                arg.append(encore, data[encore])
                                method = METHOD.POST
                                break;
                        }
                    }
            
                    doHttpRequest(small.dataset.geturlafter,arg , (text) => {
                        let jsonFile = JSON.parse(text)
                        if (jsonFile) {
                            pushElement(small, jsonFile)
                            setUrl(small)
                            for (form of small.querySelectorAll("form"))
                                setForm(form)
                        }
                    }, (e) => { viewInformation("tipical error", e, TYPE_INFO.ERROR) }, method)





                    small.dataset[name] = data[name]
                }
                else if (small.tagName === "INPUT")
                    small.value = data[name]
                else {
                    if (small.firstChild) 
                        small.firstChild.remove()
                    small.appendChild(document.createTextNode(data[name]))
                }
        }
    }
}

/**
 * 
 * @param {string} url target url
 * @param {FormData} argument  
 * @param {function} callBackOnOk if the request success
 * @param {function} callBackOnError if the request error
 * @param {METHOD} method get or post i think ☺
 * @returns 
 */
function doHttpRequest(url = "", argument = new FormData(), callBackOnOk = defaultCall, callBackOnError = (e) => { viewInformation("tipical error", e, TYPE_INFO.ERROR) }, method = METHOD.POST) {
    if (typeof callBackOnOk !== typeof callBackOnError !== 'function' && typeof url != 'string' && !method in METHOD && typeof argument !== 'object') {
        LOG_ERROR.BAD_ARGUMENT()
        return
    }

    let httpRequest = new XMLHttpRequest();
    let chargeload = true;
    httpRequest.onreadystatechange = () => {

        switch (httpRequest.readyState) {
            case httpRequest.UNSENT:
            case httpRequest.OPENED:
            case httpRequest.HEADERS_RECEIVED:
            case httpRequest.LOADING:
                setTimeout(() => {
                    if (chargeload && document.body)
                        document.body.dataset.load = ""
                }, 500)
                break;
            case httpRequest.DONE:
                chargeload = false
                if (document.body)
                    delete document.body.dataset.load
                if (httpRequest.status >= 200 && httpRequest.status < 300) {
                    callBackOnOk(httpRequest.responseText)
                }
                else {
                    callBackOnError(httpRequest.responseText)
                }
                break;
        }
    }
    httpRequest.open(method, url + constantPath, true);
    httpRequest.send(argument)
}

/**
 * Test the connexion
 */
function getFirstConnect() {
    doHttpRequest('user/connect', null, setUser, (e) => {
        console.log(e)
    }, METHOD.GET)
}

/**
 * it is the name function
 */
function disconnect() {
    doHttpRequest('user/disconnect', new FormData(), delUser, (e) => {
        viewInformation("Disconnect Error", e, TYPE_INFO.ERROR)
    }, METHOD.GET)
}

/**
 * generate web page user
 * @param {string} text a formated text as json 
 */
function setUser(text) {
    let jsonFile = JSON.parse(text)
    if (jsonFile && jsonFile.authority && jsonFile.path) {
        if (location.hash === "#login") {
            location.hash = "home"
        }
        constantPath = "/" + jsonFile.path
        user = jsonFile.authority
        doOnCharge(() => { document.body.classList.add(user) })
        getUrl(document.querySelector("#left"))
    }
    else if (jsonFile.code) {
        destructInformation()
        switch (jsonFile.code) {

        }
    }
    else {

    }
}

/**
 * disconnect user and destruct information with message
 * @param {string} message 
 */
function delUser(message) {
    viewInformation("Succes disconnect", message, TYPE_INFO.VALID)
    destructInformation()
}

/**
 * destructe all personnal information of web page
 */
function destructInformation() {
    function doThat() {
        constantPath = ""
        document.body.dataset.level = LEVEL.NONE
        location.hash = "login"
        document.querySelectorAll("[data-copy=\"true\"]").forEach((e) => { e.remove() })
    }
    doOnCharge(doThat)
}

window.onload = (e) => {
    for (let form of document.forms)
        setForm(form)
    setUrl(document.body)
    getUrl(document.body.querySelector(location.hash))
    document.getElementById("info").addEventListener("click", () => {
        delete document.body.dataset.alert
    })
}

/**
 * set the submit event listeneur
 * @param {object} form form dom element
 */
function setForm(form)
{
    form.addEventListener('submit', (e) => {
        e.preventDefault();
        let yo = (e) = {}

        switch (form.dataset.callback) {
            case "setuser": yo = (e) => {
                form.reset()
                setUser(e)
            }
                break;
            case "actualise": yo = (e) => {
                form.reset()
                getUrl(document.body.querySelector(location.hash))
            }
                break;
            default: defaultCall
                break;
        }
        console.log(yo)
        doHttpRequest(form.action, new FormData(form), yo, (e) => {
            viewInformation("Connect Error", e, TYPE_INFO.ERROR)
        })
    })
}

/**
 * set click event listeneur on all dataset-seturl to do some action 
 * @param {object} element dom element 
 */
function setUrl(element) {
    for (let item of element.querySelectorAll("[data-seturl]"))
        item.addEventListener('click', (e) => {
            e.preventDefault();
            let yo = (e) = {}
            let arg = new FormData()
            let method = METHOD.GET
            for (let encore in item.dataset) {
                switch (encore) {
                    case "callback": switch (item.dataset.callback) {
                        case "deluser": yo = delUser
                            break;
                        case "kickme": yo = (e) => {
                            item.parentNode.parentNode.remove()
                        }
                        break;
                        case "linkthis": yo = (e) => {
                            window.open("https://"+location.hostname+":6080/vnc_lite.html#"+e, "VNC viewer");
                        }
                            break;
                            case "actualise": yo = (e) => {
                                form.reset()
                                getUrl(document.body.querySelector(location.hash))
                            }
                        default: yo = defaultCall
                            break;
                    }
                        break;
                    case "seturl": break;
                    case "geturl": break;
                    default:

                        console.log(encore)
                        arg.append(encore, item.dataset[encore])
                        method = METHOD.POST
                        break;
                }
            }


            doHttpRequest(item.dataset.seturl, arg, yo, (e) => {
                viewInformation("Connect Error", e, TYPE_INFO.ERROR)
            }, method)
        })
}

getFirstConnect()

setInterval(() => {
    doHttpRequest("user/actualise", new FormData(), (e) => { }, (e) => {
        viewInformation("Your are not connected, please try it", e, TYPE_INFO.ERROR)
    }, METHOD.GET)
}, 1000 * 60 * 1)