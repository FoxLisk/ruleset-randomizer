document.addEventListener('DOMContentLoaded', function () {
    const ALLOWED = {
        'bg': 'teal-300',
        'from': 'from-teal-400',
        'to': 'to-teal-400',
        'muted': 'bg-teal-100/50',
    };

    const DISALLOWED = {
        'bg': 'red-300',
        'from': 'from-red-400',
        'to': 'to-red-400',
        'muted': 'bg-red-100/50',
    };

    const UNSPECIFIED = {
        'bg': 'bg-zinc-300',
        'from': 'from-zinc-300',
        'to': 'to-zinc-300',
        'muted': 'bg-zinc-300',
    };

    const CLASSES = {
        'ALLOWED': ALLOWED,
        'DISALLOWED': DISALLOWED,
        'UNSPECIFIED': UNSPECIFIED,
    };

    function all_classes() {
        /*
             Object.entries on a dict gives you [ [k, v] ]
             we're going down one level, so first we peel off the values,
             giving us a list of dicts again, of which we peel off the values again
        */
        return Object.entries(CLASSES).map(
            ([k, v]) => v
        ).flatMap(
            Object.entries
        ).map(([k, v]) => v);
    }


    const DORMANT_COMPARISON_CLASSES = ['bg-violet-400', 'hover:bg-violet-500'];
    const ACTIVE_COMPARISON_CLASSES = ['bg-emerald-400', 'hover:bg-emerald-500'];

    let button_template = document.getElementById('comparison-button'),
        comparisons_container = document.getElementById('comparisons-list');
    var _comparisons = {};

    function indicate_active_comparison_button(active) {
        for (let button of comparisons_container.querySelectorAll('button')) {
            if (button.dataset.name === active) {
                button.classList.remove(...DORMANT_COMPARISON_CLASSES);
                button.classList.add(...ACTIVE_COMPARISON_CLASSES);
            } else {
                button.classList.remove(...ACTIVE_COMPARISON_CLASSES);
                button.classList.add(...DORMANT_COMPARISON_CLASSES);
            }
        }
    }

    function clear() {
        let comp = _comparisons['Weekly'];
        for (let d of document.querySelectorAll('.rule-row')) {
            let rule_name = d.dataset.name,
                allowed = comp[rule_name];
            d.classList.remove(...all_classes());
            d.querySelector('.allowed').textContent = allowed;
            d.classList.add(CLASSES[allowed]['bg']);
            d.querySelector('.comparison-slot').classList.add('hidden');
        }
        indicate_active_comparison_button('None');
    }

    fetch('/comparisons')
        .then(response => response.json())
        .then(function (comparisons) {
            for (let comp of comparisons) {
                _comparisons[comp.name] = comp;
                if (comp.name === 'Weekly') {
                    continue;
                }
                let button = button_template.content.cloneNode(true);
                button.querySelector('button').textContent = comp.name;
                button.querySelector('button').dataset.name = comp.name;
                comparisons_container.appendChild(button);
            }
            let clear_button = button_template.content.cloneNode(true);
            clear_button.querySelector('button').textContent = 'None';
            clear_button.querySelector('button').dataset.name = 'None';
            clear_button.querySelector('button').classList.add(...ACTIVE_COMPARISON_CLASSES);
            comparisons_container.appendChild(clear_button);


            comparisons_container.addEventListener('click', function (e) {
                e.preventDefault();
                let comp_name = e.target.dataset.name;
                if (!comp_name) {
                    console.log('No name, probably missed the button');
                    return;
                }
                if (comp_name === 'None') {
                    clear();
                    return;
                }
                let comp = _comparisons[comp_name];
                if (!comp) {
                    console.log('No comparison found ? ??');
                    return;
                }

                for (let d of document.querySelectorAll('.rule-row')) {

                    let rule_name = d.dataset.name,
                        allowed_in_comparison = comp[rule_name],
                        allowed_in_comparee = d.querySelector('.comparison-target').dataset.allowed,
                        is_same = allowed_in_comparison === allowed_in_comparee;


                    d.querySelector('.allowed').textContent = allowed_in_comparison;
                    d.classList.remove(...all_classes());

                    if (is_same) {
                        d.classList.add(
                            CLASSES[allowed_in_comparee]['muted']
                        );
                    } else {
                        d.classList.add('bg-gradient-to-r', CLASSES[allowed_in_comparison]['from'], CLASSES[allowed_in_comparee]['to']);
                    }
                    d.querySelector('.comparison-slot').classList.remove('hidden');
                }

                indicate_active_comparison_button(comp_name);
            });
        });
});