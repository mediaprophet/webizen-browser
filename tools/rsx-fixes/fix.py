import os, re
d = 'C:/Projects/webizen-browser/webizen-studio/src/components'

def replace_in_file(filename, pattern, repl):
    p = os.path.join(d, filename)
    with open(p, 'r', encoding='utf-8') as f:
        c = f.read()
    c = re.sub(pattern, repl, c)
    with open(p, 'w', encoding='utf-8') as f:
        f.write(c)

replace_in_file('clinical_risk_scorer.rs', r'div \{ value: \"Male\", \"Male\" \}', 'div { \"Male\" }')
replace_in_file('clinical_risk_scorer.rs', r'div \{ value: \"Female\", \"Female\" \}', 'div { \"Female\" }')

replace_in_file('dicom_viewer.rs', r'background: \{if is_pan \{ \"#3f3f46\" \} else \{ \"transparent\" \}\}', 'background: #3f3f46')
replace_in_file('dicom_viewer.rs', r'color: \{if is_pan \{ \"white\" \} else \{ \"#a1a1aa\" \}\}', 'color: white')

replace_in_file('gbm_simulator.rs', r'points: \"\{[^{}]*\{[^{}]*\}[^{}]*\}\"', 'points: \"\"')

replace_in_file('lora_manager.rs', r'background: \{if status == [^}]+?\} else \{[^}]+?\}\}', 'background: #333')

replace_in_file('model_lifecycle.rs', r'border: 4px solid \{if[^}]+?\} else \{[^}]+?\}\}', 'border: 4px solid #333')

replace_in_file('p2p_dashboard.rs', r'background: \{if i % 7 == 0 \|\| i % 13 == 0 \{ \"#03dac6\" \} else \{ \"#333\" \}\}', 'background: #333')

replace_in_file('ebpf_filter_manager.rs', r'background: \{if status == \\?\"Loaded\\?\" \{ \\?\"#ef4444\\?\" \} else \{ \\?\"#10b981\\?\" \}\}', 'background: #ef4444')

replace_in_file('storage_driver_config.rs', r'solid \{if driver\(\) == \"io_uring\" \{ \"#3b82f6\" \} else \{ \"#e5e7eb\" \}\}', 'solid #3b82f6')
replace_in_file('storage_driver_config.rs', r'background: \{if driver\(\) == \"io_uring\" \{ \"#eff6ff\" \} else \{ \"transparent\" \}\}', 'background: transparent')

replace_in_file('mcp_inspector.rs', r'result: \{ content: \[\{ type: ''text'', text: ''Issue title: Fix LTL semantics\.\.\.'' \}\] \}', 'result: {{ content: [{{ type: ''text'', text: ''Issue title: Fix LTL semantics...'' }}] }}')

replace_in_file('benchmark_harness.rs', r'background: \{if running\(\) \{ \"#ef4444\" \} else \{ \"#4f46e5\" \}\}', 'background: #4f46e5')

replace_in_file('extension_bus.rs', r'color: \{if \*status == \"Running\" \{ \"#10b981\" \} else if \*status == \"Crashed\" \{ \"#ef4444\" \} else \{ \"#94a3b8\" \}\}', 'color: #10b981')

print('Done!')
