"use strict";

const DATA_URL = "rust_30day_plan_2026-04-26.json";

const CATEGORY_DEFS = [
  {
    id: "review",
    label: "復習・整理",
    keywords: ["復習", "まとめ", "週", "総復習", "到達判定", "次計画", "詰まり記録", "整理", "メモ"],
  },
  {
    id: "testing",
    label: "テスト",
    keywords: ["test", "テスト", "clippy", "fixture", "benchmark", "手動確認"],
  },
  {
    id: "cli",
    label: "CLI",
    keywords: ["cli", "minigrep", "grep", "parser", "subcommand", "clap", "env::args", "引数", "wc", "head", "find", "help", "release build"],
  },
  {
    id: "io",
    label: "I/O・低レイヤ",
    keywords: ["file", "i/o", "linux", "pathbuf", "metadata", "permissions", "symlink", "directory", "process", "command", "pipe", "stdin", "stdout", "buffering", "streaming", "fd", "/proc", "std::os::unix", "低レイヤ", "proc", "signal", "大入力", "memory", "walkdir", "探索"],
  },
  {
    id: "reading",
    label: "読解",
    keywords: ["読解", "oss", "cargo tree", "cargo doc", "crate構造読む", "小crate読む", "追跡", "call graph", "ripgrep"],
  },
  {
    id: "concept",
    label: "概念",
    keywords: ["book", "所有権", "move", "借用", "string", "vec", "result", "option", "closure", "fn/", "trait", "generics", "where", "associated", "dyn", "iterator", "map/filter", "collect", "smart pointer", "box", "rc", "refcell", "borrow", "raii", "drop", "lifetime", "arc", "mutex", "thread", "channel", "concurrency", "unsafe", "raw pointer", "soundness", "zst", "allocation", "pointer", "rustonomicon", "safe wrapper", "drop glue"],
  },
  {
    id: "design",
    label: "設計・文書",
    keywords: ["設計", "docs", "readme", "public api", "crate構成", "module", "lib/bin", "config", "要件定義", "契約", "default値", "api"],
  },
  {
    id: "practice",
    label: "実装・演習",
    keywords: ["演習", "練習", "実装", "作成", "改造", "修正", "写経", "模写", "課題", "仕上げ", "強化", "解き直し", "自力", "比較", "導入", "適用", "追加", "改善", "整備", "refactor"],
  },
];

const PHASES = [
  { id: "week1", label: "Week 1 基礎固め", range: "Day 1-7", from: 1, to: 7 },
  { id: "week2", label: "Week 2 CLIとライブラリ", range: "Day 8-14", from: 8, to: 14 },
  { id: "week3", label: "Week 3 Linux/OSS読解", range: "Day 15-21", from: 15, to: 21 },
  { id: "week4", label: "Week 4 仕上げと低レイヤ", range: "Day 22-28", from: 22, to: 28 },
  { id: "final", label: "Final 到達判定", range: "Day 29-30", from: 29, to: 30 },
];

const state = {
  plan: null,
  query: "",
  phase: "all",
  category: "all",
  density: "normal",
  dimDone: false,
  done: new Set(),
};

const els = {};

document.addEventListener("DOMContentLoaded", () => {
  cacheElements();
  bindEvents();
  loadPlan();
});

function cacheElements() {
  for (const id of [
    "app",
    "planRange",
    "planTitle",
    "statusPanel",
    "content",
    "overviewCopy",
    "progressText",
    "todayText",
    "progressFill",
    "searchInput",
    "phaseSelect",
    "categorySelect",
    "dimDoneInput",
    "printButton",
    "resetButton",
    "legend",
    "weekStats",
    "successList",
    "timeline",
    "resourceList",
  ]) {
    els[id] = document.getElementById(id);
  }
}

function bindEvents() {
  els.searchInput.addEventListener("input", (event) => {
    state.query = event.target.value.trim().toLowerCase();
    render();
  });

  els.phaseSelect.addEventListener("change", (event) => {
    state.phase = event.target.value;
    render();
  });

  els.categorySelect.addEventListener("change", (event) => {
    state.category = event.target.value;
    render();
  });

  els.dimDoneInput.addEventListener("change", (event) => {
    state.dimDone = event.target.checked;
    render();
  });

  document.querySelectorAll("[data-density]").forEach((button) => {
    button.addEventListener("click", () => {
      state.density = button.dataset.density;
      document.querySelectorAll("[data-density]").forEach((item) => {
        item.classList.toggle("is-active", item === button);
      });
      els.app.classList.toggle("is-compact", state.density === "compact");
    });
  });

  els.printButton.addEventListener("click", () => window.print());

  els.resetButton.addEventListener("click", () => {
    if (!state.plan || state.done.size === 0) {
      return;
    }
    const ok = window.confirm("記録した完了状態をすべて消します。");
    if (!ok) {
      return;
    }
    state.done.clear();
    saveDone();
    render();
  });
}

async function loadPlan() {
  const bundledPlan = getBundledPlan();
  if (bundledPlan) {
    initializePlan(bundledPlan);
    return;
  }

  try {
    const response = await fetch(DATA_URL, { cache: "no-store" });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    initializePlan(await response.json());
  } catch (error) {
    renderFileFallback(error);
  }
}

function getBundledPlan() {
  if (window.RUST_30DAY_PLAN_DATA) {
    return window.RUST_30DAY_PLAN_DATA;
  }

  const embedded = document.getElementById("planData");
  const text = embedded?.textContent?.trim();
  return text ? JSON.parse(text) : null;
}

function renderFileFallback(error) {
  clearChildren(els.statusPanel);

  const strong = document.createElement("strong");
  strong.textContent = "JSONを自動読み込みできませんでした。";
  els.statusPanel.append(strong);

  const detail = document.createElement("p");
  detail.textContent = `同じフォルダでローカルサーバを起動するか、下から ${DATA_URL} を選択してください。`;
  els.statusPanel.append(detail);

  const small = document.createElement("p");
  small.className = "overview-copy";
  small.textContent = `原因: ${error.message}`;
  els.statusPanel.append(small);

  const loader = document.createElement("label");
  loader.className = "file-loader";
  loader.textContent = "JSONを選択";

  const input = document.createElement("input");
  input.type = "file";
  input.accept = "application/json,.json";
  input.addEventListener("change", async (event) => {
    const file = event.target.files?.[0];
    if (!file) {
      return;
    }
    try {
      initializePlan(JSON.parse(await file.text()));
    } catch (fileError) {
      small.textContent = `読み込み失敗: ${fileError.message}`;
    }
  });

  loader.append(input);
  els.statusPanel.append(loader);
}

function initializePlan(plan) {
  validatePlan(plan);
  state.plan = plan;
  state.done = loadDone(plan);
  els.statusPanel.hidden = true;
  els.content.hidden = false;
  renderStaticPlan();
  render();
}

function validatePlan(plan) {
  if (!plan || !Array.isArray(plan.days) || plan.days.length === 0) {
    throw new Error("days 配列が見つかりません。");
  }
  for (const day of plan.days) {
    if (!Number.isInteger(day.day) || !Array.isArray(day.hours)) {
      throw new Error("day または hours の形式が不正です。");
    }
  }
}

function renderStaticPlan() {
  const plan = state.plan;
  const first = dateForDay(1);
  const last = dateForDay(plan.days.length);
  const totalHours = getAllHours().length;
  const dailyHours = plan.assumptions?.daily_study_hours ?? 10;

  els.planTitle.textContent = plan.title ?? "Rust 30日学習計画";
  els.planRange.textContent = `${formatDate(first)} - ${formatDate(last)} / ${totalHours}h`;
  els.overviewCopy.textContent = `${plan.days.length}日間、毎日${dailyHours}時間。主目標は「${plan.assumptions?.primary_goal ?? "Rust学習"}」。休憩目安は H${(plan.break_policy?.default_break_after_hours ?? []).join(" / H")} 後。`;

  renderSelects();
  renderLegend();
  renderSuccessCriteria();
  renderResources();
}

function renderSelects() {
  clearChildren(els.phaseSelect);
  clearChildren(els.categorySelect);

  els.phaseSelect.append(new Option("すべて", "all"));
  for (const phase of PHASES) {
    els.phaseSelect.append(new Option(`${phase.label} (${phase.range})`, phase.id));
  }

  els.categorySelect.append(new Option("すべて", "all"));
  for (const category of CATEGORY_DEFS) {
    els.categorySelect.append(new Option(category.label, category.id));
  }
}

function renderLegend() {
  clearChildren(els.legend);
  for (const category of CATEGORY_DEFS) {
    const item = document.createElement("span");
    item.className = `legend-item category-${category.id}`;

    const dot = document.createElement("span");
    dot.className = "legend-dot";
    dot.setAttribute("aria-hidden", "true");

    const label = document.createElement("span");
    label.textContent = category.label;

    item.append(dot, label);
    els.legend.append(item);
  }
}

function renderSuccessCriteria() {
  clearChildren(els.successList);
  for (const criterion of state.plan.success_criteria ?? []) {
    const item = document.createElement("li");
    item.textContent = criterion;
    els.successList.append(item);
  }
}

function renderResources() {
  clearChildren(els.resourceList);
  for (const resource of state.plan.resources ?? []) {
    const link = document.createElement("a");
    link.className = "resource-link";
    link.href = resource.url;
    link.target = "_blank";
    link.rel = "noreferrer";
    link.textContent = resource.name;
    els.resourceList.append(link);
  }
}

function render() {
  renderProgress();
  renderWeekStats();
  renderTimeline();
}

function renderProgress() {
  const allHours = getAllHours();
  const completed = allHours.filter(({ day, hour }) => isDone(day.day, hour.hour)).length;
  const percent = allHours.length ? (completed / allHours.length) * 100 : 0;
  const today = dayNumberForToday();

  els.progressText.textContent = `${completed} / ${allHours.length}h`;
  els.progressFill.style.width = `${percent}%`;
  els.todayText.textContent = today ? `Today Day ${today}` : "期間外";
}

function renderWeekStats() {
  clearChildren(els.weekStats);

  for (const phase of PHASES) {
    const days = state.plan.days.filter((day) => day.day >= phase.from && day.day <= phase.to);
    const counts = new Map(CATEGORY_DEFS.map((category) => [category.id, 0]));
    let total = 0;

    for (const day of days) {
      for (const hour of day.hours) {
        const category = categorizeTask(hour.task);
        counts.set(category.id, (counts.get(category.id) ?? 0) + 1);
        total += 1;
      }
    }

    const row = document.createElement("div");
    row.className = "week-row";

    const name = document.createElement("div");
    name.className = "week-name";
    name.textContent = phase.label;

    const bar = document.createElement("div");
    bar.className = "stacked-bar";

    for (const category of CATEGORY_DEFS) {
      const count = counts.get(category.id) ?? 0;
      if (count === 0) {
        continue;
      }
      const segment = document.createElement("span");
      segment.className = `stacked-segment category-${category.id}`;
      segment.style.setProperty("--w", `${(count / total) * 100}%`);
      segment.title = `${category.label}: ${count}h`;
      bar.append(segment);
    }

    const hours = document.createElement("div");
    hours.className = "week-total";
    hours.textContent = `${total}h`;

    row.append(name, bar, hours);
    els.weekStats.append(row);
  }
}

function renderTimeline() {
  clearChildren(els.timeline);
  let renderedDays = 0;

  for (const phase of PHASES) {
    if (state.phase !== "all" && state.phase !== phase.id) {
      continue;
    }

    const matchingDays = state.plan.days.filter((day) => {
      return day.day >= phase.from && day.day <= phase.to && dayMatchesFilters(day);
    });

    if (matchingDays.length === 0) {
      continue;
    }

    renderedDays += matchingDays.length;
    els.timeline.append(renderPhaseBlock(phase, matchingDays));
  }

  if (renderedDays === 0) {
    const empty = document.createElement("p");
    empty.className = "no-results";
    empty.textContent = "条件に一致する時間枠がありません。";
    els.timeline.append(empty);
  }
}

function renderPhaseBlock(phase, days) {
  const section = document.createElement("section");
  section.className = "phase-block";

  const heading = document.createElement("div");
  heading.className = "phase-heading";

  const title = document.createElement("h2");
  title.textContent = phase.label;

  const meta = document.createElement("span");
  meta.className = "phase-meta";
  meta.textContent = `${phase.range} / ${days.length}日`;

  heading.append(title, meta);
  section.append(heading);

  const list = document.createElement("div");
  list.className = "day-list";
  for (const day of days) {
    list.append(renderDayCard(day));
  }
  section.append(list);

  return section;
}

function renderDayCard(day) {
  const article = document.createElement("article");
  article.className = "day-card";

  const head = document.createElement("div");
  head.className = "day-head";

  const titleBlock = document.createElement("div");
  const title = document.createElement("h3");
  title.className = "day-title";
  title.textContent = `Day ${day.day}`;
  const date = document.createElement("div");
  date.className = "day-date";
  date.textContent = formatDate(dateForDay(day.day));
  titleBlock.append(title, date);

  const focus = document.createElement("div");
  focus.className = "day-focus";
  focus.textContent = day.hours.map((hour) => hour.task).slice(0, 4).join(" / ");
  focus.title = day.hours.map((hour) => `H${hour.hour} ${hour.task}`).join(" / ");

  const doneCount = day.hours.filter((hour) => isDone(day.day, hour.hour)).length;
  const progress = document.createElement("div");
  progress.className = "day-progress";
  progress.textContent = `${doneCount}/${day.hours.length}h`;

  head.append(titleBlock, focus, progress);
  article.append(head);

  const grid = document.createElement("div");
  grid.className = "hour-grid";
  for (const hour of day.hours) {
    grid.append(renderHourCell(day, hour));
  }
  article.append(grid);

  return article;
}

function renderHourCell(day, hour) {
  const category = categorizeTask(hour.task);
  const done = isDone(day.day, hour.hour);
  const matches = hourMatchesFilters(hour);
  const breakAfter = state.plan.break_policy?.default_break_after_hours ?? [];

  const cell = document.createElement("button");
  cell.type = "button";
  cell.className = `hour-cell category-${category.id}`;
  cell.classList.toggle("is-done", done);
  cell.classList.toggle("is-muted", !matches);
  cell.classList.toggle("is-dimmed-done", state.dimDone);
  cell.classList.toggle("break-after", breakAfter.includes(hour.hour));
  cell.setAttribute("aria-pressed", String(done));
  cell.title = `Day ${day.day} H${hour.hour}: ${hour.task}`;
  cell.addEventListener("click", () => {
    toggleDone(day.day, hour.hour);
  });

  const meta = document.createElement("span");
  meta.className = "hour-meta";

  const hourLabel = document.createElement("span");
  hourLabel.textContent = `H${hour.hour}`;

  const doneLabel = document.createElement("span");
  doneLabel.textContent = done ? "完了" : "";

  meta.append(hourLabel, doneLabel);

  const task = document.createElement("span");
  task.className = "hour-task";
  task.textContent = hour.task;

  const categoryLabel = document.createElement("span");
  categoryLabel.className = "hour-category";
  categoryLabel.textContent = category.label;

  cell.append(meta, task, categoryLabel);
  return cell;
}

function dayMatchesFilters(day) {
  return day.hours.some((hour) => hourMatchesFilters(hour));
}

function hourMatchesFilters(hour) {
  const category = categorizeTask(hour.task);
  const categoryOk = state.category === "all" || state.category === category.id;
  const queryOk = !state.query || `${hour.task} h${hour.hour}`.toLowerCase().includes(state.query);
  return categoryOk && queryOk;
}

function categorizeTask(task) {
  const text = String(task).toLowerCase();
  return CATEGORY_DEFS.find((category) => {
    return category.keywords.some((keyword) => text.includes(keyword.toLowerCase()));
  }) ?? CATEGORY_DEFS[CATEGORY_DEFS.length - 1];
}

function getAllHours() {
  return state.plan.days.flatMap((day) => day.hours.map((hour) => ({ day, hour })));
}

function progressKey(day, hour) {
  return `day-${day}:hour-${hour}`;
}

function storageKey(plan = state.plan) {
  return `rust-30day-plan-progress:${plan.created_date ?? "unknown"}`;
}

function loadDone(plan) {
  try {
    const raw = window.localStorage.getItem(storageKey(plan));
    return new Set(raw ? JSON.parse(raw) : []);
  } catch {
    return new Set();
  }
}

function saveDone() {
  window.localStorage.setItem(storageKey(), JSON.stringify([...state.done]));
}

function isDone(day, hour) {
  return state.done.has(progressKey(day, hour));
}

function toggleDone(day, hour) {
  const key = progressKey(day, hour);
  if (state.done.has(key)) {
    state.done.delete(key);
  } else {
    state.done.add(key);
  }
  saveDone();
  render();
}

function dayNumberForToday() {
  const today = startOfDay(new Date());
  const start = dateForDay(1);
  const diff = Math.round((today - start) / 86400000);
  const day = diff + 1;
  return day >= 1 && day <= state.plan.days.length ? day : null;
}

function dateForDay(dayNumber) {
  const [year, month, day] = (state.plan.created_date ?? "2026-04-26").split("-").map(Number);
  const date = new Date(year, month - 1, day);
  date.setDate(date.getDate() + dayNumber - 1);
  return startOfDay(date);
}

function startOfDay(date) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function formatDate(date) {
  return new Intl.DateTimeFormat("ja-JP", {
    month: "numeric",
    day: "numeric",
    weekday: "short",
  }).format(date);
}

function clearChildren(element) {
  while (element.firstChild) {
    element.firstChild.remove();
  }
}
