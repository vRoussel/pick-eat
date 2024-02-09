export function insert_sorted(array, el, cmp) {
    if (array.length == 0) {
        array.push(el)
        return
    }

    let min = 0
    let max = array.length - 1
    let i = Math.floor((min + max) / 2)

    if (cmp(el, array[max]) >= 0) {
        array.push(el)
        return
    }

    while (min < max) {
        if (cmp(el, array[i]) < 0) {
            max = i
        } else {
            min = i + 1
        }
        i = Math.floor((min + max) / 2)
    }
    array.splice(i, 0, el)
}

export function isOverflown(element) {
    let tolerance = 3
    if (element == null) return false
    return (
        element.clientHeight + tolerance < element.scrollHeight ||
        element.clientWidth + tolerance < element.scrollWidth
    )
}

export function handle_form_api_errors(api_answer, errors_map, toast_elem) {
    if (!Object.hasOwn(api_answer.data, 'errors')) return
    let popup_shown = false
    api_answer.data.errors.forEach((error) => {
        if (Object.hasOwn(error, 'field') && error.field !== null) {
            errors_map[error.field] = error.message
        } else {
            toast_elem.show_error(error.message)
            popup_shown = true
        }
    })
    if (!popup_shown) {
        toast_elem.show_error('Veuillez corriger les erreurs et réessayer')
    }
}

export function handle_form_local_errors(errors, errors_map, toast_elem) {
    errors.forEach((error) => {
        errors_map[error.path] = error.message
    })
    toast_elem.show_error('Veuillez corriger les erreurs et réessayer')
}

export function loadScript(src, callback) {
    var script = document.createElement('script')
    script.src = src
    if (callback) script.onload = callback
    var doc = document.head || document.body
    doc.appendChild(script)
}

export function qty_scaled(base_qty, scaling_factor, tick_size) {
    let scaled = base_qty * scaling_factor
    let ticks = scaled / tick_size
    let ticks_rounded = Math.round(ticks)
    return ticks_rounded * tick_size
}

