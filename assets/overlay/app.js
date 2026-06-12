async function update() {
    try {
        const response =
            await fetch(
                "http://127.0.0.1:9876/status.json"
            );

        const data =
            await response.json();

        document.getElementById(
            "status"
        ).textContent =
            data.ready
                ? "READY TO RECORD"
                : "NOT READY";

        document.getElementById(
            "quality"
        ).textContent =
            `QUALITY: ${data.quality}`;

        document.getElementById(
            "zone"
        ).textContent =
            data.target_zone_ok
                ? "TARGET ZONE: IDEAL"
                : "TARGET ZONE: ADJUST";
    } catch {
        document.getElementById(
            "status"
        ).textContent =
            "API OFFLINE";
    }
}

update();

setInterval(
    update,
    1000
);
