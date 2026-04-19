const { invoke } = window.__TAURI__.core;

// DOM Elements
const views = {
    "archive": document.getElementById("view-archive"),
    "terminal": document.getElementById("view-terminal"),
    "metrics": document.getElementById("view-metrics"),
    "chat": document.getElementById("view-chat"),
};

const taskContainer = document.getElementById("task-container");
const archiveContainer = document.getElementById("archive-container");
const modal = document.getElementById("create-task-modal");
const closeBtn = document.getElementById("close-modal-btn");
const createForm = document.getElementById("create-task-form");

const editModal = document.getElementById("edit-task-modal");
const closeEditBtn = document.getElementById("close-edit-modal-btn");
const editForm = document.getElementById("edit-task-form");

// Active Session Card Elements
const activeSessionCard = document.querySelector(".bg-surface-container-lowest.border-l-2.border-cyber-lime");
const activeTimerDisplay = activeSessionCard.querySelector(".text-5xl");
const activeProgressBar = activeSessionCard.querySelector(".bg-cyber-lime.h-full");

// State
let tasks = [];
let activeTimerInterval = null;
let currentActiveTask = null;
let currentView = "terminal";

// View Navigation
document.querySelectorAll('nav a').forEach(link => {
    link.addEventListener('click', (e) => {
        e.preventDefault();
        const iconSpan = link.querySelector('span.material-symbols-outlined');
        if (!iconSpan) return;
        
        const icon = iconSpan.textContent;
        let targetView = null;
        
        if (icon === 'database') targetView = 'archive';
        else if (icon === 'terminal') targetView = 'terminal';
        else if (icon === 'query_stats') targetView = 'metrics';
        else if (icon === 'forum') targetView = 'chat';
        
        if (targetView && views[targetView]) {
            switchView(targetView);
            
            // Update active state on nav links
            document.querySelectorAll('nav a').forEach(navLink => {
                navLink.classList.remove('bg-[#c3f5ff]', 'text-black', 'scale-100');
                navLink.classList.add('text-[#353534]', 'hover:text-[#c3f5ff]', 'hover:bg-[#1c1b1b]');
            });
            
            link.classList.add('bg-[#c3f5ff]', 'text-black', 'scale-100');
            link.classList.remove('text-[#353534]', 'hover:text-[#c3f5ff]', 'hover:bg-[#1c1b1b]');
        }
    });
});

function switchView(viewName) {
    Object.keys(views).forEach(key => {
        if (views[key]) {
            if (key === viewName) {
                views[key].classList.remove('hidden');
                if (key === 'terminal') views[key].classList.add('grid');
                else views[key].classList.add('flex');
            } else {
                views[key].classList.add('hidden');
                if (key === 'terminal') views[key].classList.remove('grid');
                else views[key].classList.remove('flex');
            }
        }
    });
    currentView = viewName;
    renderTasks();
}

async function fetchTasks() {
    try {
        tasks = await invoke("get_all_tasks_command");
        renderTasks();
        updateActiveSessionCard();
        updateMetrics();
    } catch (e) {
        console.error("Failed to fetch tasks", e);
    }
}

function renderTasks() {
    if (currentView === "terminal") {
        taskContainer.innerHTML = "";
        
        const inProgress = tasks.filter(t => t.status === "InProgress");
        const pending = tasks.filter(t => t.status === "Todo" || t.status === "Paused");
        
        if (inProgress.length === 0 && pending.length === 0) {
            taskContainer.innerHTML = `<div class="p-4 text-on-surface-variant font-mono text-sm text-center">No active or pending tasks.</div>`;
        } else {
            renderGroup(inProgress, taskContainer);
            renderGroup(pending, taskContainer);
        }
    } else if (currentView === "archive") {
        archiveContainer.innerHTML = "";
        const completed = tasks.filter(t => t.status === "Completed");
        
        if (completed.length === 0) {
            archiveContainer.innerHTML = `<div class="p-4 text-on-surface-variant font-mono text-sm text-center">No completed tasks yet.</div>`;
        } else {
            renderGroup(completed, archiveContainer);
        }
    }
}

function renderGroup(group, container) {
    group.forEach(task => {
        const isActive = task.status === "InProgress";
        const icon = isActive ? "pause_circle" : (task.status === "Completed" ? "check_circle" : "play_circle");
        const titleClass = isActive ? "text-cyber-lime" : (task.status === "Completed" ? "text-muted-slate line-through" : "text-on-surface");
        const bgClass = isActive ? "bg-surface-container-low hover:bg-surface-container" : (task.status === "Completed" ? "bg-transparent opacity-60 border border-surface-container-lowest" : "bg-[#0a0a0a] border border-surface-container-low hover:bg-surface-container-lowest");

        const taskEl = document.createElement("div");
        taskEl.className = `p-4 flex items-center group transition-colors duration-150 ${bgClass}`;
        
        taskEl.innerHTML = `
            <button class="mr-4 transition-colors ${isActive ? 'text-cyber-lime hover:text-primary' : 'text-on-surface-variant hover:text-primary'}" onclick="toggleTimer('${task.id}', '${task.status}')">
                <span class="material-symbols-outlined text-2xl" data-icon="${icon}" ${isActive ? 'data-weight="fill" style="font-variation-settings: \'FILL\' 1;"' : ''}>${icon}</span>
            </button>
            <div class="flex-1 cursor-pointer" onclick="openEditModal('${task.id}')" title="Click to edit">
                <h3 class="font-mono text-sm ${titleClass} mb-1">${task.title}</h3>
                <p class="font-body text-xs text-on-surface-variant line-clamp-1">${task.description || "No description"}</p>
            </div>
            <div class="flex flex-col items-end space-y-2 ml-4">
                <span class="bg-surface-container-highest px-2 py-0.5 text-[10px] font-mono text-on-surface">${task.category.toUpperCase().substring(0, 3)}</span>
                <span class="font-mono text-xs ${isActive ? 'text-cyber-lime' : 'text-on-surface-variant'}">${formatTime(task.actual_duration_secs)} / ${task.estimated_duration_mins}m</span>
            </div>
            <div class="flex flex-col ml-4 space-y-2 md:opacity-0 md:group-hover:opacity-100 opacity-100 transition-opacity">
                ${task.status !== 'Completed' ? `<button class="text-cyber-lime hover:text-primary" onclick="markTaskDone('${task.id}')" title="Mark Done"><span class="material-symbols-outlined text-sm">done</span></button>` : ''}
                <button class="text-error hover:text-red-400" onclick="deleteTask('${task.id}')" title="Delete"><span class="material-symbols-outlined text-sm">delete</span></button>
            </div>
        `;
        container.appendChild(taskEl);
    });
}

function formatTime(seconds) {
    if (!seconds) return "00:00";
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    if (h > 0) {
        return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    }
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
}

async function updateActiveSessionCard() {
    const activeTask = tasks.find(t => t.status === "InProgress");
    
    if (activeTask) {
        if (currentActiveTask?.id !== activeTask.id) {
            currentActiveTask = activeTask;
            const timerStatus = await invoke("get_timer_status_command", { taskId: activeTask.id });
            if (timerStatus) {
                activeTask.sessionStartTime = new Date(timerStatus.start_time);
            }
            
            // New task started, fetch initial AI pulse and set interval for every 5 minutes
            triggerAI();
            if (aiPulseInterval) clearInterval(aiPulseInterval);
            aiPulseInterval = setInterval(triggerAI, 5 * 60 * 1000);
        }
        
        activeSessionCard.style.display = "flex";
        activeSessionCard.querySelector("span").textContent = activeTask.title;
        
        startUITimer(activeTask);
    } else {
        activeSessionCard.style.display = "none";
        stopUITimer();
        currentActiveTask = null;
        triggerAI(); // Will clear the text to standing by and clear the interval
    }
}

function startUITimer(task) {
    stopUITimer();
    
    const updateDisplay = () => {
        let elapsed = task.actual_duration_secs || 0;
        if (task.sessionStartTime) {
             elapsed += Math.floor((new Date() - task.sessionStartTime) / 1000);
        }
        
        const h = Math.floor(elapsed / 3600);
        const m = Math.floor((elapsed % 3600) / 60);
        const s = elapsed % 60;
        
        let timeStr = "";
        if (h > 0) {
            timeStr = `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}<span class="opacity-50">:${s.toString().padStart(2, '0')}</span>`;
        } else {
            timeStr = `${m.toString().padStart(2, '0')}<span class="opacity-50">:${s.toString().padStart(2, '0')}</span>`;
        }
        
        activeTimerDisplay.innerHTML = timeStr;
        
        const estSecs = task.estimated_duration_mins * 60;
        const progress = Math.min(100, (elapsed / estSecs) * 100);
        activeProgressBar.style.width = `${progress}%`;
        
        if (progress > 100) {
            activeProgressBar.classList.remove("bg-cyber-lime");
            activeProgressBar.classList.add("bg-error");
            activeTimerDisplay.classList.remove("text-cyber-lime");
            activeTimerDisplay.classList.add("text-error");
        } else {
            activeProgressBar.classList.add("bg-cyber-lime");
            activeProgressBar.classList.remove("bg-error");
            activeTimerDisplay.classList.add("text-cyber-lime");
            activeTimerDisplay.classList.remove("text-error");
        }
    };
    
    updateDisplay();
    activeTimerInterval = setInterval(updateDisplay, 1000);
}

function stopUITimer() {
    if (activeTimerInterval) {
        clearInterval(activeTimerInterval);
        activeTimerInterval = null;
    }
}


window.toggleTimer = async (taskId, currentStatus) => {
    try {
        if (currentStatus === "InProgress") {
            await invoke("stop_timer_command", { taskId });
        } else {
            // First stop any other running tasks
            const active = tasks.find(t => t.status === "InProgress");
            if (active) {
                 await invoke("stop_timer_command", { taskId: active.id });
            }
            await invoke("start_timer_command", { taskId });
        }
        await fetchTasks();
    } catch (e) {
        console.error("Failed to toggle timer", e);
    }
};

window.deleteTask = async (taskId) => {
    if (confirm("Delete this task?")) {
        try {
            await invoke("delete_task_command", { taskId });
            await fetchTasks();
        } catch (e) {
            console.error("Failed to delete task", e);
        }
    }
};

function updateMetrics() {
    const total = tasks.length;
    let totalSecs = 0;
    let estSecs = 0;
    
    tasks.forEach(t => {
        totalSecs += t.actual_duration_secs;
        estSecs += (t.estimated_duration_mins * 60);
    });
    
    document.getElementById("metric-total").textContent = total;
    document.getElementById("metric-duration").textContent = formatTime(totalSecs);
    
    const efficiency = estSecs > 0 ? Math.round((estSecs / totalSecs) * 100) : 0;
    const effSpan = document.getElementById("metric-efficiency");
    
    if (totalSecs === 0) {
        effSpan.textContent = "-";
    } else {
        effSpan.textContent = efficiency + "%";
        if (efficiency >= 100) {
            effSpan.classList.remove("text-[#ffdf96]", "text-error");
            effSpan.classList.add("text-cyber-lime");
        } else if (efficiency < 50) {
            effSpan.classList.remove("text-[#ffdf96]", "text-cyber-lime");
            effSpan.classList.add("text-error");
        } else {
            effSpan.classList.add("text-[#ffdf96]");
            effSpan.classList.remove("text-cyber-lime", "text-error");
        }
    }
}

// AI Pulse Logic
let aiPulseInterval = null;

async function triggerAI() {
    const active = tasks.find(t => t.status === "InProgress");
    const textSpan = document.getElementById("ai-pulse-text");
    
    if (!textSpan) return;

    if (!active) {
        textSpan.textContent = "System standing by. Initiate a task to receive AI telemetry.";
        if (aiPulseInterval) {
            clearInterval(aiPulseInterval);
            aiPulseInterval = null;
        }
        return;
    }
    
    try {
        textSpan.textContent = "Analyzing kinetic flow with local LLM...";
        
        const est = active.estimated_duration_mins;
        const act = Math.floor(active.actual_duration_secs / 60);
        
        const prompt = `I am currently working on a task called "${active.title}" (Category: ${active.category}). I estimated ${est} minutes and have spent ${act} minutes so far. Give me a single short, punchy sentence of advice or encouragement as an AI operator. Do not use quotes, formatting, or markdown. Keep it under 15 words.`;
        
        const response = await invoke("chat_with_ai_command", { 
            history: [{ role: "user", content: prompt }] 
        });
        
        textSpan.textContent = response.content.replace(/["*]/g, '').trim();
    } catch (e) {
        console.error("AI Error:", e);
        textSpan.textContent = "Local AI unavailable or failed to respond.";
    }
}

// Call triggerAI when active task changes (managed inside updateActiveSessionCard)

// Modal Logic
function openModal() {
    modal.classList.remove("hidden");
    document.getElementById("task-title").focus();
}

function closeModal() {
    modal.classList.add("hidden");
    createForm.reset();
}

closeBtn.addEventListener("click", closeModal);

createForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    try {
        await invoke("create_task_command", {
            title: document.getElementById("task-title").value,
            description: document.getElementById("task-desc").value || null,
            category: document.getElementById("task-category").value,
            estimatedDurationMins: parseInt(document.getElementById("task-est").value)
        });
        closeModal();
        await fetchTasks();
    } catch (e) {
        console.error("Failed to create task", e);
        alert("Error creating task.");
    }
});

// Edit Modal Logic
window.openEditModal = (taskId) => {
    const task = tasks.find(t => t.id === taskId);
    if (!task) return;
    
    document.getElementById("edit-task-id").value = task.id;
    document.getElementById("edit-task-title").value = task.title;
    document.getElementById("edit-task-desc").value = task.description || "";
    document.getElementById("edit-task-category").value = task.category;
    document.getElementById("edit-task-est").value = task.estimated_duration_mins;
    
    editModal.classList.remove("hidden");
    document.getElementById("edit-task-title").focus();
};

function closeEditModal() {
    editModal.classList.add("hidden");
    editForm.reset();
}

if (closeEditBtn) {
    closeEditBtn.addEventListener("click", closeEditModal);
}

if (editForm) {
    editForm.addEventListener("submit", async (e) => {
        e.preventDefault();
        try {
            await invoke("update_task_command", {
                taskId: document.getElementById("edit-task-id").value,
                title: document.getElementById("edit-task-title").value,
                description: document.getElementById("edit-task-desc").value || null,
                category: document.getElementById("edit-task-category").value,
                estimatedDurationMins: parseInt(document.getElementById("edit-task-est").value)
            });
            closeEditModal();
            await fetchTasks();
        } catch (e) {
            console.error("Failed to update task", e);
            alert("Error updating task.");
        }
    });
}

window.markTaskDone = async (taskId) => {
    try {
        await invoke("update_task_command", {
            taskId: taskId,
            status: "Completed"
        });
        await fetchTasks();
    } catch (e) {
        console.error("Failed to mark task done", e);
    }
};


// Achievements Logic
const analyzeBtn = document.getElementById("analyze-achievements-btn");
const analyzeOutput = document.getElementById("achievements-output");

if (analyzeBtn && analyzeOutput) {
    analyzeBtn.addEventListener("click", async () => {
        analyzeBtn.disabled = true;
        analyzeBtn.textContent = "ANALYZING...";
        analyzeOutput.textContent = "Pinging local Gemma 4 model for analysis...";
        
        try {
            const analysis = await invoke("analyze_achievements_command");
            analyzeOutput.textContent = analysis;
        } catch (e) {
            console.error("Failed to analyze achievements", e);
            analyzeOutput.textContent = "Error: Could not retrieve analysis. Ensure Ollama is running and model is available.";
        } finally {
            analyzeBtn.disabled = false;
            analyzeBtn.textContent = "ANALYZE";
        }
    });
}

// Chat Logic
let chatHistory = [];
const chatContainer = document.getElementById("chat-history");
const chatInput = document.getElementById("chat-input");
const chatSendBtn = document.getElementById("send-chat-btn");
const clearChatBtn = document.getElementById("clear-chat-btn");

function appendChatMessage(role, content) {
    const msgDiv = document.createElement("div");
    const isUser = role === "user";
    
    msgDiv.className = `p-3 font-mono text-sm max-w-[80%] ${isUser ? 'bg-surface-container-high self-end border-r-2 border-primary ml-auto' : 'bg-surface-container-low self-start border-l-2 border-cyber-lime mr-auto'}`;
    
    const roleSpan = document.createElement("div");
    roleSpan.className = `text-[10px] uppercase tracking-widest mb-1 ${isUser ? 'text-primary text-right' : 'text-cyber-lime'}`;
    roleSpan.textContent = isUser ? "OPERATOR" : "GEMMA 4";
    
    const contentSpan = document.createElement("div");
    contentSpan.className = "text-on-surface whitespace-pre-wrap";
    contentSpan.textContent = content;
    
    msgDiv.appendChild(roleSpan);
    msgDiv.appendChild(contentSpan);
    
    // Remove the placeholder message if it's the first real message
    const placeholder = chatContainer.querySelector(".text-center.text-on-surface-variant");
    if (placeholder) {
        placeholder.remove();
    }
    
    chatContainer.appendChild(msgDiv);
    chatContainer.scrollTop = chatContainer.scrollHeight;
}

async function sendChatMessage() {
    const text = chatInput.value.trim();
    if (!text) return;
    
    // UI update for user
    chatInput.value = "";
    appendChatMessage("user", text);
    chatSendBtn.disabled = true;
    chatSendBtn.textContent = "WAIT...";
    
    // Add to history
    chatHistory.push({ role: "user", content: text });
    
    try {
        const response = await invoke("chat_with_ai_command", { history: chatHistory });
        
        // Add to history and UI
        chatHistory.push(response);
        appendChatMessage(response.role, response.content);
    } catch (e) {
        console.error("Chat error", e);
        appendChatMessage("system", "ERROR: Could not connect to local AI endpoint.");
    } finally {
        chatSendBtn.disabled = false;
        chatSendBtn.textContent = "SEND";
        chatInput.focus();
    }
}

if (chatSendBtn && chatInput) {
    chatSendBtn.addEventListener("click", sendChatMessage);
    chatInput.addEventListener("keydown", (e) => {
        if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            sendChatMessage();
        }
    });
}

if (clearChatBtn) {
    clearChatBtn.addEventListener("click", () => {
        chatHistory = [];
        chatContainer.innerHTML = '<div class="text-center text-on-surface-variant font-mono text-xs my-4">Chat session initialized. Send a message to begin.</div>';
    });
}

async function initApp() {
    // Modify the header layout for the add button
    const headerRow = document.querySelector(".flex.justify-between.items-end.border-b-2.border-surface-container-low.pb-2");
    
    // Check if we already added the button to prevent duplicates on hot reloads
    if (headerRow && !headerRow.querySelector("button")) {
        const actionDiv = document.createElement("div");
        actionDiv.className = "flex space-x-2";
        
        const btn = document.createElement("button");
        btn.className = "bg-primary text-[#131313] px-3 py-1 text-[10px] font-mono uppercase tracking-widest hover:bg-primary-container transition-colors font-bold";
        btn.innerText = "+ INITIATE";
        btn.onclick = openModal;
        
        actionDiv.appendChild(btn);
        
        // Replace the priority span with our action div
        const prioritySpan = headerRow.querySelector("span");
        if(prioritySpan) prioritySpan.remove();
        headerRow.appendChild(actionDiv);
    }

    // AI Trigger button
    const aiBtn = document.querySelector(".bg-surface-container-highest.hover\\:bg-surface-bright");
    if (aiBtn) {
        aiBtn.innerText = "Trigger Pulse";
        aiBtn.onclick = triggerAI;
    }

    // Ensure terminal view is visible initially
    switchView("terminal");

    await fetchTasks();
}

if (document.readyState === 'loading') {
    window.addEventListener("DOMContentLoaded", initApp);
} else {
    initApp();
}