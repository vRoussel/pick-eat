export const PLACEHOLDER_IMG = require('@/assets/camera.svg')

function str_without_accents(str) {
    return str.normalize('NFD').replace(/\p{Diacritic}/gu, "")
}

export function obj_with_searchable_name(obj, src_field_name) {
    return {
        ...obj,
        searchable_name: obj[src_field_name] + '#' + str_without_accents(obj[src_field_name])
    }
}

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
    let tolerance = 5
    if (element == null)
        return false
    return element.clientHeight + tolerance < element.scrollHeight || element.clientWidth + tolerance < element.scrollWidth;
}

export function random_str(len) {
    return (Math.random()+1).toString(36).substring(2,len+2)
}
