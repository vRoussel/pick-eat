export function insert_sorted(array, el, cmp) {
    if (array.length == 0) {
        array.push(el);
        return;
    }

    let min = 0;
    let max = array.length - 1;
    let i = Math.floor((min + max) / 2);

    if (cmp(el, array[max]) >= 0) {
        array.push(el);
        return;
    }

    while (min < max) {
        if (cmp(el, array[i]) < 0) {
            max = i;
        } else {
            min = i + 1;
        }
        i = Math.floor((min + max) / 2);
    }
    array.splice(i, 0, el);
}

export function isOverflown(element) {
    let tolerance = 3
    if (element == null)
        return false
    return element.clientHeight + tolerance < element.scrollHeight || element.clientWidth + tolerance < element.scrollWidth;
}
