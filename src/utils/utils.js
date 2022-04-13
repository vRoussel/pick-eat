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
    console.log(i)
    array.splice(i, 0, el);
}
