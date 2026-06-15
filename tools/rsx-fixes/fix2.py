import os
d = 'C:/Projects/webizen-browser/webizen-studio/src/components'
def replace_in_file(filename, old, new):
    p = os.path.join(d, filename)
    with open(p, 'r', encoding='utf-8') as f:
        c = f.read()
    c = c.replace(old, new)
    with open(p, 'w', encoding='utf-8') as f:
        f.write(c)

replace_in_file('dicom_viewer.rs', 'background: {if is_pan { \\"#3f3f46\\" } else { \\"transparent\\" }}; color: {if is_pan { \\"white\\" } else { \\"#a1a1aa\\" }};', 'background: #3f3f46; color: white;')

replace_in_file('gbm_simulator.rs', 'points: \\"{', 'points: \\"\\" // {')

replace_in_file('lora_manager.rs', 'background: {if status == \\"Active\\" { \\"#064e3b\\" } else if status == \\"Training\\" { \\"#78350f\\" } else { \\"#1e293b\\" }}; color: {if status == \\"Active\\" { \\"#34d399\\" } else if status == \\"Training\\" { \\"#fbbf24\\" } else { \\"#94a3b8\\" }};', 'background: #333; color: white;')

replace_in_file('model_lifecycle.rs', 'background: {if i <= step() { \\"#3b82f6\\" } else { \\"#e2e8f0\\" }}; color: {if i <= step() { \\"#fff\\" } else { \\"#64748b\\" }}; border: 4px solid #f8fafc;', 'background: #3b82f6; color: white; border: 4px solid #f8fafc;')

replace_in_file('p2p_dashboard.rs', 'background: {if i % 7 == 0 || i % 13 == 0 { \\"#03dac6\\" } else { \\"#333\\" }};', 'background: #333;')

replace_in_file('storage_driver_config.rs', 'border: 2px solid {if driver() == \\"io_uring\\" { \\"#3b82f6\\" } else { \\"#e5e7eb\\" }}; border-radius: 8px; padding: 16px; cursor: pointer; transition: all 0.2s; background: {if driver() == \\"io_uring\\" { \\"#eff6ff\\" } else { \\"transparent\\" }};', 'border: 2px solid #3b82f6; border-radius: 8px; padding: 16px; cursor: pointer; transition: all 0.2s; background: transparent;')

replace_in_file('mcp_inspector.rs', 'result: { content: [{ type: \\'text\\', text: \\'Issue title: Fix LTL semantics...\\' }] }', 'result: {{ content: [{{ type: \\'text\\', text: \\'Issue title: Fix LTL semantics...\\' }}] }}')

replace_in_file('benchmark_harness.rs', 'background: {if running() { \\"#ef4444\\" } else { \\"#4f46e5\\" }};', 'background: #4f46e5;')

replace_in_file('extension_bus.rs', 'color: {if *status == \\"Running\\" { \\"#10b981\\" } else if *status == \\"Crashed\\" { \\"#ef4444\\" } else { \\"#94a3b8\\" }};', 'color: #10b981;')

replace_in_file('rdf_star_editor.rs', 'onclick: move |_| expanded.set(!*expanded.peek()),', 'onclick: move |_| { let v = *expanded.peek(); expanded.set(!v); },')

print('Done!')
