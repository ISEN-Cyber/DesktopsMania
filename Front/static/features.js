/* set action of all element in page */
window.addEventListener("load", (e) => {
    itemsRow = document.getElementsByClassName("itemsRow")
    for (x = 0; x < itemsRow.length; x = x + 1) {
        let item = itemsRow.item(x)
        let add=0
        let op=0
        item.addEventListener('mousedown', ((e) => {
            if (e.layerX < 50)
                add = -1
            else if (e.layerX > item.clientWidth - 50)
                add = 1
            op= 3
            setTimeout(decal,10)
        }))
        item.addEventListener('mouseup', ((e) => {
            add=0
        }))
        function decal()
        {
            item.scrollLeft +=  add*(op=Math.pow(op,1.01))
            if(add != 0)
                setTimeout(decal,10)
        }
    }
})