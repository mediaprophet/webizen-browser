import os
d = 'C:/Projects/webizen-browser/webizen-studio/src/components'
def replace_line(filename, old, new):
    p = os.path.join(d, filename)
    with open(p, 'r', encoding='utf-8') as f:
        c = f.read()
    c = c.replace(old, new)
    with open(p, 'w', encoding='utf-8') as f:
        f.write(c)

replace_line('benchmark_harness.rs',
             'style: "background: {if running() { \\"#ef4444\\" } else { \\"#4f46e5\\" }}; color: white; font-size: 20px; font-weight: bold; padding: 16px 48px; border: none; border-radius: 50px; cursor: pointer; box-shadow: 0 10px 25px -5px rgba(79, 70, 229, 0.5); transition: all 0.3s;",',
             'style: "background: #4f46e5; color: white; font-size: 20px; font-weight: bold; padding: 16px 48px; border: none; border-radius: 50px; cursor: pointer; box-shadow: 0 10px 25px -5px rgba(79, 70, 229, 0.5); transition: all 0.3s;",')

replace_line('extension_bus.rs',
             'span { style: "font-size: 14px; color: #64748b;", "Status: ", b { style: "color: {if *status == \\"Running\\" { \\"#10b981\\" } else if *status == \\"Crashed\\" { \\"#ef4444\\" } else { \\"#94a3b8\\" }};", "{status}" } }',
             'span { style: "font-size: 14px; color: #64748b;", "Status: ", b { style: "color: #10b981;", "{status}" } }')

replace_line('rdf_star_editor.rs',
             'onclick: move |_| expanded.set(!*expanded.peek()),',
             'onclick: move |_| { let v = *expanded.peek(); expanded.set(!v); },')

replace_line('storage_driver_config.rs',
             'label { style: "flex: 1; border: 2px solid {if driver() == \\"io_uring\\" { \\"#3b82f6\\" } else { \\"#e5e7eb\\" }}; border-radius: 8px; padding: 16px; cursor: pointer; transition: all 0.2s; background: {if driver() == \\"io_uring\\" { \\"#eff6ff\\" } else { \\"transparent\\" }};",',
             'label { style: "flex: 1; border: 2px solid #3b82f6; border-radius: 8px; padding: 16px; cursor: pointer; transition: all 0.2s; background: transparent;",')

replace_line('storage_driver_config.rs',
             'label { style: "flex: 1; border: 2px solid {if driver() == \\"direct_io\\" { \\"#3b82f6\\" } else { \\"#e5e7eb\\" }}; border-radius: 8px; padding: 16px; cursor: pointer; transition: all 0.2s; background: {if driver() == \\"direct_io\\" { \\"#eff6ff\\" } else { \\"transparent\\" }};",',
             'label { style: "flex: 1; border: 2px solid #e5e7eb; border-radius: 8px; padding: 16px; cursor: pointer; transition: all 0.2s; background: transparent;",')

replace_line('mcp_inspector.rs',
             'div { style: "color: #abb2bf; padding-left: 16px;", "result: { content: [{ type: \\'text\\', text: \\'Issue title: Fix LTL semantics...\\' }] }" }',
             'div { style: "color: #abb2bf; padding-left: 16px;", "result: {{ content: [{{ type: \\'text\\', text: \\'Issue title: Fix LTL semantics...\\' }}] }}" }')

replace_line('p2p_dashboard.rs',
             'div { style: "height: 20px; background: {if i % 7 == 0 || i % 13 == 0 { \\"#03dac6\\" } else { \\"#333\\" }}; border-radius: 2px;", title: "Bucket {i}" }',
             'div { style: "height: 20px; background: #333; border-radius: 2px;", title: "Bucket {i}" }')

replace_line('model_lifecycle.rs',
             'div { style: "width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: bold; background: {if i <= step() { \\"#3b82f6\\" } else { \\"#e2e8f0\\" }}; color: {if i <= step() { \\"#fff\\" } else { \\"#64748b\\" }}; border: 4px solid #f8fafc;",',
             'div { style: "width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: bold; background: #3b82f6; color: white; border: 4px solid #f8fafc;",')

replace_line('lora_manager.rs',
             'span { style: "padding: 4px 8px; border-radius: 12px; font-size: 12px; background: {if status == \\"Active\\" { \\"#064e3b\\" } else if status == \\"Training\\" { \\"#78350f\\" } else { \\"#1e293b\\" }}; color: {if status == \\"Active\\" { \\"#34d399\\" } else if status == \\"Training\\" { \\"#fbbf24\\" } else { \\"#94a3b8\\" }};", "{status}" }',
             'span { style: "padding: 4px 8px; border-radius: 12px; font-size: 12px; background: #064e3b; color: #34d399;", "{status}" }')

replace_line('ebpf_filter_manager.rs',
             '"{if status == \\"Loaded\\" { \\"Unload BPF\\" } else { \\"Load BPF\\" }}"',
             '"Load BPF"')

print("Done")
