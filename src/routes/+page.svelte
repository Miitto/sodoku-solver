<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let array = $state(Array(81));
    let original = $state(Array(81));
    let solutions = $state(Array(0));
    let curSolution = $state(0);

    // prettier-ignore
    array = [
        4, 8, null, 6, null, null, 5, null, null,
        5, null, 2, null, 9, null, 4, 6, null,
        null, 7, null, 4, 5, null, 2, null, null,
        1, null, 5, 9, null, 4, null, 2, 6,
        null, 2, null, 5, null, null, 9, 4, null,
        null, 4, null, 2, null, null, 1, 5, 3,
        null, 6, 4, null, null, 5, null, 9, 2,
        null, null, null, null, 4, 2, 6, null, 5,
        2, 5, null, 8, 6, 9, null, 1, 4,
    ]

    original = array.slice();

    function ensureSingleDigit() {
        for (let i = 0; i < array.length; i++) {
            const val = array[i];
            if (!val) continue;
            if (val > 9) {
                const str = val.toString();
                array[i] = parseInt(str.charAt(str.length - 1));
            }
            if (val < 1) {
                array[i] = null;
            }
        }
    }

    function onInput(event: Event, x: number, y: number) {
        const str = (event.target as HTMLInputElement).value;
        const lastChar = str.charAt(str.length - 1);
        if (!lastChar) return;
        if (lastChar < "1" || lastChar > "9") {
            (event.target as HTMLInputElement).value = "";
        } else {
            (event.target as HTMLInputElement).value = lastChar;
        }

        array[x * 9 + y] = parseInt((event.target as HTMLInputElement).value);
        original[x * 9 + y] = array[x * 9 + y];
    }

    async function solve() {
        ensureSingleDigit();
        curSolution = 0;
        const sols: number[][] = await invoke("solve_sudoku", {
            cells: array,
        });

        solutions = sols;

        if (sols.length > 0) {
            const solved = sols[0];
            original = array.slice();

            array = solved;
        } else {
            alert("No solution found");
        }
    }

    function nextSol() {
        if (curSolution < solutions.length - 1) {
            curSolution += 1;
            array = solutions[curSolution];
        }
    }

    function prevSol() {
        if (curSolution > 0) {
            curSolution -= 1;
            array = solutions[curSolution];
        }
    }
</script>

<div class="flex items-center flex-col p-g">
    {#each Array(9) as _, i}
        <div
            class="row flex border-b first:border-t-[3px] border-foreground w-fit"
        >
            {#each Array(9) as _, j}
                <div
                    class="cell border-r border-foreground aspect-square first:border-l-[3px]"
                >
                    <input
                        class:solved={array[i * 9 + j] !== original[i * 9 + j]}
                        class="w-[min(calc(80vw_/_9),_calc(80vh_/_9))] h-full text-center text-lg bg-background focus-visible:ring-0 focus-visible:outline-null focus-within:invert"
                        type="number"
                        bind:value={array[i * 9 + j]}
                        oninput={(e) => onInput(e, i, j)}
                    />
                </div>
            {/each}
        </div>
    {/each}
    <button
        class="px-4 py-2 m-4 bg-primary rounded-lg text-primary-foreground"
        onclick={solve}>Solve</button
    >
</div>
<button onclick={() => (array = original)}>Reset</button>
<button onclick={() => (array = Array(81).fill(null))}>Clear</button>

<button
    onclick={prevSol}
    disabled={curSolution <= 0 || solutions.length == 0}
    >Previous Solution</button
>
<button
    onclick={nextSol}
    disabled={curSolution >= solutions.length - 1}>Next Solution</button
>

<style>
    .cell:nth-child(3n) {
        border-right-width: 3px;
    }
    .row:nth-child(3n) {
        border-bottom-width: 3px;
    }

    .solved {
        color: red;
    }
</style>
