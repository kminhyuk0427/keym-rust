const { invoke } = window.__TAURI__.tauri;

let config = null;
let selectedMacroIndex = 0;
let selectedActionIndex = null;
let availableKeys = [];
let editingAction = false;

// 초기화
async function init() {
    try {
        availableKeys = await invoke('get_available_keys');
        config = await invoke('load_config');
        populateKeySelects();
        refreshUI();
        setStatus('로드 완료');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 키 선택 드롭다운 채우기
function populateKeySelects() {
    const selects = [document.getElementById('triggerKey'), document.getElementById('modalKey')];
    selects.forEach(select => {
        select.innerHTML = '';
        availableKeys.forEach(key => {
            const option = document.createElement('option');
            option.value = key;
            option.textContent = key;
            select.appendChild(option);
        });
    });
}

// UI 새로고침
function refreshUI() {
    refreshMacroTabs();
    refreshMacroSettings();
    refreshActionsTable();
}

// 매크로 탭 새로고침
function refreshMacroTabs() {
    const container = document.getElementById('macroTabs');
    container.innerHTML = '';
    
    config.macros.forEach((macro, index) => {
        const tab = document.createElement('button');
        tab.className = 'macro-tab';
        if (index === selectedMacroIndex) {
            tab.classList.add('active');
        }
        tab.textContent = `매크로 ${index + 1} [${macro.trigger}] (${macro.actions.length}개)`;
        tab.onclick = () => selectMacro(index);
        container.appendChild(tab);
    });
}

// 매크로 설정 새로고침
function refreshMacroSettings() {
    if (selectedMacroIndex < config.macros.length) {
        const macro = config.macros[selectedMacroIndex];
        document.getElementById('triggerKey').value = macro.trigger;
        document.getElementById('mode').value = macro.mode;
    }
}

// 액션 테이블 새로고침
function refreshActionsTable() {
    const tbody = document.getElementById('actionsBody');
    tbody.innerHTML = '';
    
    if (selectedMacroIndex >= config.macros.length) return;
    
    const actions = config.macros[selectedMacroIndex].actions;
    actions.forEach((action, index) => {
        const row = document.createElement('tr');
        if (index === selectedActionIndex) {
            row.classList.add('selected');
        }
        row.onclick = () => selectAction(index);
        
        row.innerHTML = `
            <td>${action.hold_ms}</td>
            <td>${action.key}</td>
            <td>${action.delay_ms}</td>
            <td>
                <button class="action-btn" onclick="editAction(${index})">수정</button>
                <button class="action-btn" onclick="deleteAction(${index})">삭제</button>
            </td>
        `;
        tbody.appendChild(row);
    });
}

// 매크로 선택
function selectMacro(index) {
    selectedMacroIndex = index;
    selectedActionIndex = null;
    refreshUI();
    setStatus(`선택: 매크로 ${index + 1}`);
}

// 액션 선택
function selectAction(index) {
    selectedActionIndex = index;
    refreshActionsTable();
}

// 매크로 추가
async function addMacro() {
    try {
        config = await invoke('add_macro', { config });
        selectedMacroIndex = config.macros.length - 1;
        selectedActionIndex = null;
        refreshUI();
        setStatus('새 매크로 추가됨');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 매크로 저장
async function saveMacro() {
    try {
        const trigger = document.getElementById('triggerKey').value;
        const mode = parseInt(document.getElementById('mode').value);
        
        config = await invoke('update_macro', {
            config,
            index: selectedMacroIndex,
            trigger,
            mode
        });
        
        refreshUI();
        setStatus('매크로 설정 저장됨');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 매크로 삭제
async function deleteMacro() {
    if (!confirm('정말 이 매크로를 삭제하시겠습니까?')) return;
    
    try {
        config = await invoke('delete_macro', {
            config,
            index: selectedMacroIndex
        });
        
        if (config.macros.length === 0) {
            selectedMacroIndex = 0;
        } else if (selectedMacroIndex >= config.macros.length) {
            selectedMacroIndex = config.macros.length - 1;
        }
        
        selectedActionIndex = null;
        refreshUI();
        setStatus('매크로 삭제됨');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 전체 저장
async function saveAll() {
    try {
        await invoke('save_config', { config });
        setStatus('전체 저장 완료');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 액션 추가 모달 열기
function openAddActionModal() {
    if (config.macros.length === 0) {
        setStatus('매크로를 먼저 생성하세요');
        return;
    }
    
    editingAction = false;
    document.getElementById('modalTitle').textContent = '액션 추가';
    document.getElementById('modalKey').value = availableKeys[0];
    document.getElementById('modalHold').value = 50;
    document.getElementById('modalDelay').value = 50;
    document.getElementById('actionModal').classList.add('active');
}

// 액션 수정
function editAction(index) {
    editingAction = true;
    selectedActionIndex = index;
    
    const action = config.macros[selectedMacroIndex].actions[index];
    document.getElementById('modalTitle').textContent = '액션 수정';
    document.getElementById('modalKey').value = action.key;
    document.getElementById('modalHold').value = action.hold_ms;
    document.getElementById('modalDelay').value = action.delay_ms;
    document.getElementById('actionModal').classList.add('active');
}

// 액션 저장 (추가/수정)
async function saveAction() {
    try {
        const key = document.getElementById('modalKey').value;
        const holdMs = parseInt(document.getElementById('modalHold').value);
        const delayMs = parseInt(document.getElementById('modalDelay').value);
        
        if (editingAction) {
            config = await invoke('update_action', {
                config,
                macroIndex: selectedMacroIndex,
                actionIndex: selectedActionIndex,
                key,
                holdMs,
                delayMs
            });
            setStatus('액션 수정됨');
        } else {
            config = await invoke('add_action', {
                config,
                macroIndex: selectedMacroIndex,
                key,
                holdMs,
                delayMs
            });
            setStatus('액션 추가됨');
        }
        
        closeModal();
        refreshUI();
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 액션 삭제
async function deleteAction(index) {
    try {
        config = await invoke('delete_action', {
            config,
            macroIndex: selectedMacroIndex,
            actionIndex: index
        });
        
        selectedActionIndex = null;
        refreshUI();
        setStatus('액션 삭제됨');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 액션 이동
async function moveAction(direction) {
    if (selectedActionIndex === null) {
        setStatus('이동할 액션을 선택하세요');
        return;
    }
    
    const actions = config.macros[selectedMacroIndex].actions;
    const newIndex = direction === 'up' ? selectedActionIndex - 1 : selectedActionIndex + 1;
    
    if (newIndex < 0 || newIndex >= actions.length) {
        setStatus(direction === 'up' ? '이미 맨 위입니다' : '이미 맨 아래입니다');
        return;
    }
    
    try {
        config = await invoke('move_action', {
            config,
            macroIndex: selectedMacroIndex,
            fromIndex: selectedActionIndex,
            toIndex: newIndex
        });
        
        selectedActionIndex = newIndex;
        refreshUI();
        setStatus('액션 이동됨');
    } catch (error) {
        setStatus(`오류: ${error}`);
    }
}

// 모달 닫기
function closeModal() {
    document.getElementById('actionModal').classList.remove('active');
}

// 상태 메시지 설정
function setStatus(message) {
    document.getElementById('status').textContent = `상태: ${message}`;
}

// 이벤트 리스너
document.getElementById('addMacroBtn').onclick = addMacro;
document.getElementById('saveMacroBtn').onclick = saveMacro;
document.getElementById('deleteMacroBtn').onclick = deleteMacro;
document.getElementById('saveAllBtn').onclick = saveAll;
document.getElementById('addActionBtn').onclick = openAddActionModal;
document.getElementById('moveUpBtn').onclick = () => moveAction('up');
document.getElementById('moveDownBtn').onclick = () => moveAction('down');
document.getElementById('modalSaveBtn').onclick = saveAction;
document.getElementById('modalCancelBtn').onclick = closeModal;

// 앱 초기화
init();