<svelte:head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <meta name="google-site-verification" content="I28dxke1LaLkwkxT-PgsNbiy1rOQAe-iRs-YsQnzgh0" />
  
  <title>モーラカウンター - 日本語テキストのモーラ数測定ツール</title>
  <meta name="description" content="日本語のテキストを入力するだけで、モーラ数を簡単にカウントできる無料のWebツールです。" />
  <meta name="keywords" content="モーラ,モーラ数,カウンター,日本語,プレゼン,文字数,発表" />
  
  <meta property="og:title" content="モーラカウンター" />
  <meta property="og:description" content="日本語テキストのモーラ数を測定するツール" />
  <meta property="og:type" content="website" />
  <meta property="og:url" content="https://murase-hijiri.github.io/mora-counter-web/" />
</svelte:head>

<script lang="ts">
    import { onMount } from "svelte";

    let wasmModule: any = null;
    let isLoading = true;
    let error = "";

    let inputText = "";
    let moraCount = 0;
    let estimatedTime = 0;

    let moraPerMinute = 350;

    onMount(async () => {
        try {
            const module = await import("$lib/wasm/wasm_mora_counter.js");
            await module.default();

            module.init_vibrato();

            wasmModule = module;
            isLoading = false;
        } catch (e) {
            error = `Failed to load WASM: ${e}`;
            console.error(e);
            isLoading = false;
        }
    });

    function handleCount() {
        if (!wasmModule) return;

        try {
            moraCount = wasmModule.count_mora(inputText);
            estimatedTime = moraCount / moraPerMinute;
        } catch (e) {
            error = `Error counting mora: ${e}`;
            console.error(e);
        }
    }

    // 時間を「00分00秒」形式にフォーマット
    function formatTime(minutes: number): string {
        const mins = Math.floor(minutes);
        const secs = Math.floor((minutes - mins) * 60);
        if (mins < 1) {
            return `${String(secs).padStart(1, "0")}秒`;
        } else {
            return `${String(mins).padStart(1, "0")}分${String(secs).padStart(1, "0")}秒`;
        }
    }

    $: if (wasmModule && (inputText || inputText === "")) {
        console.log("Handle Count!");
        handleCount();
    }
</script>

<main>
    <h1>モーラカウンター</h1>

    {#if isLoading}
        <p>辞書を読み込み中...</p>
    {:else if error}
        <p style="color: red;">{error}</p>
    {:else}
        <div class="container">
            <div class="input-section">
                <textarea
                    bind:value={inputText}
                    placeholder="プレゼン原稿を入力してください"
                    rows="15"
                ></textarea>
            </div>

            <div class="results">
                <div class="result-item">
                    <span class="label">モーラ数:</span>
                    <span class="value">{moraCount}</span>
                </div>

                <div class="result-item">
                    <span class="label">推定時間:</span>
                    <span class="value">{formatTime(estimatedTime)}</span>
                </div>

                <div class="speed-control">
                    <label>
                        話す速度: {moraPerMinute} モーラ/分
                        <input
                            type="range"
                            bind:value={moraPerMinute}
                            min="250"
                            max="450"
                            step="10"
                        />
                    </label>
                </div>
            </div>
        </div>
    {/if}
</main>

<style>
    main {
        max-width: 1400px;
        margin: 0 auto;
        padding: 2rem;
        min-height: 100vh;
    }

    h1 {
        font-size: 2rem;
        margin-bottom: 2rem;
    }

    .container {
        display: grid;
        grid-template-columns: 1fr 350px;
        gap: 2rem;
        align-items: start;
    }

    .input-section {
        min-width: 0; /* グリッドオーバーフロー防止 */
    }

    textarea {
        width: 100%;
        min-height: 500px;
        padding: 1rem;
        font-size: 1rem;
        font-family: sans-serif;
        line-height: 1.6;
        border: 2px solid #ddd;
        border-radius: 8px;
        resize: vertical;
        box-sizing: border-box;
    }

    textarea:focus {
        outline: none;
        border-color: #4a9eff;
    }

    .results {
        position: sticky;
        top: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .result-item {
        padding: 1.5rem;
        background: #f5f5f5;
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .label {
        font-size: 0.9rem;
        color: #666;
        font-weight: 500;
    }

    .value {
        font-size: 2.5rem;
        font-weight: bold;
        color: #333;
    }

    .speed-control {
        padding: 1.5rem;
        background: #f5f5f5;
        border-radius: 8px;
    }

    .speed-control label {
        display: block;
        font-size: 0.9rem;
        color: #666;
        font-weight: 500;
    }

    input[type="range"] {
        width: 100%;
        margin-top: 0.8rem;
    }

    /* レスポンシブ対応 */
    @media (max-width: 900px) {
        .container {
            grid-template-columns: 1fr;
        }

        .results {
            position: static;
        }

        textarea {
            min-height: 300px;
        }
    }
</style>
