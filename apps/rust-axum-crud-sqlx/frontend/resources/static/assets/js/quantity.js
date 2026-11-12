document.querySelectorAll(".quantity-pill").forEach((pill) => {
    const input = pill.querySelector(".quantity-input");
    const form = pill.closest("form");
    const update =
        form && form.id ? document.querySelector(`.quantity-update[form="${form.id}"]`) : null;

    const sync = () => {
        if (update) {
            update.hidden = input.value === input.defaultValue;
        }
    };

    pill.querySelectorAll(".quantity-btn").forEach((button) => {
        button.addEventListener("click", () => {
            if (button.dataset.step === "up") {
                input.stepUp();
            } else {
                input.stepDown();
            }
            sync();
        });
    });
    input.addEventListener("input", sync);
});
