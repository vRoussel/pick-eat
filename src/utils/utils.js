function str_without_accents(str) {
    return str.normalize('NFD').replace(/\p{Diacritic}/gu, "")
}

export function obj_with_searchable_name(obj, src_field_name) {
    return {
        ...obj,
        searchable_name: obj[src_field_name] + '#' + str_without_accents(obj[src_field_name])
    }
}
