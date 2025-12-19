        let select = document.getElementById("manual-pair");
        // Bewaar de huidige geselecteerde waarde
let currentValue = select.value;

// Leeg de dropdown en voeg nieuwe opties toe
select.innerHTML = "";
pairs.forEach(p => {
    let opt = document.createElement("option");
    opt.value = p;
    opt.text = p;
    select.appendChild(opt);
});

// Herstel de eerder geselecteerde waarde als deze nog bestaat in de nieuwe lijst
if (pairs.includes(currentValue)) {
    select.value = currentValue;
}