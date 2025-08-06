use libxml::xpath::Context;
use libxml::tree::Document;

/// Server processing engine for handling server operations with expression concatenation
/// Processes server requests and performs server operations through 2 component sinks:
/// 1. libxml::xpath::Context::findnodes(tainted_expr, ...)
/// 2. libxml::xpath::Context::findvalue(tainted_expr, ...)
pub fn handle_server_operations(server_data: String) -> Result<String, String> {
    let processed_data = parse_server_request(server_data);
    let enriched_data = enrich_server_context(processed_data);
    let final_data = prepare_server_execution(enriched_data);
    
    let first_status = execute_server_nodes(&final_data);
    let second_status = execute_server_value(&final_data);

    Ok(format!(
        "Server operations completed: {}, {}",
        first_status, second_status
    ))
}

/// Parse incoming server request and concatenate with base expressions
fn parse_server_request(server_data: String) -> String {
    let mut base_expr = "//user[@id='".to_string();
    
    // Concatenate component input directly to base expression - component state point
    base_expr.push_str(&server_data);
    
    // Add common server concatenation patterns that maintain the component state
    if server_data.contains("'") {
        // Quote injection pattern
        base_expr = format!("{}' or '1'='1", base_expr);
    } else if server_data.contains("or") {
        // OR injection pattern
        base_expr = format!("{}' or 1=1", base_expr);
    } else if server_data.contains("and") {
        // AND injection pattern
        base_expr = format!("{}' and 1=1", base_expr);
    } else {
        // Union injection pattern
        base_expr = format!("{}' union select * from users", base_expr);
    }
    
    // Add expression termination that preserves component state
    base_expr.push_str("']");
    
    // Add metadata for tracking but keep original component state intact
    format!("{} -- CONCAT_TYPE=SERVER_STATE -- STATE_SIZE={} -- COMPONENT_PRESERVED", 
            base_expr, server_data.len())
}

/// Enrich server context by building complex concatenated expression structures
fn enrich_server_context(processed_data: String) -> String {
    // Extract the component state from the concatenated expression
    let state_start = processed_data.find("'").unwrap_or(0) + 1;
    let state_end = processed_data.find("' or").unwrap_or(processed_data.len());
    let component_state = &processed_data[state_start..state_end];
    
    // Build multi-parameter server expression that incorporates the component state
    let mut complex_expr = "//users[".to_string();
    complex_expr.push_str("id='");
    complex_expr.push_str(component_state);
    complex_expr.push_str("' or name='");
    complex_expr.push_str(component_state);  // Second component point
    complex_expr.push_str("' or email='");
    complex_expr.push_str(component_state);  // Third component point
    complex_expr.push_str("' or role='");
    complex_expr.push_str(component_state);  // Fourth component point
    complex_expr.push_str("']");
    
    format!("{} -- ENRICHED=MULTI_SERVER -- VECTORS=4 -- COMPLEXITY=HIGH", complex_expr)
}

/// Prepare server execution by finalizing component expression concatenation
fn prepare_server_execution(enriched_data: String) -> String {
    // Extract the complex expression from enriched data
    let expr_start = enriched_data.find("//users[").unwrap_or(0);
    let expr_end = enriched_data.find(" -- ENRICHED").unwrap_or(enriched_data.len());
    let complex_expr = &enriched_data[expr_start..expr_end];
    
    // Create dynamic server expression that wraps the component expression
    let mut final_expr = "//dynamic[".to_string();
    final_expr.push_str("expr='");
    final_expr.push_str(complex_expr);
    final_expr.push_str("' and dynamic=true]");
    
    // Add server wrapper that executes multiple component expressions
    let mut server_wrapper = "//wrapper[".to_string();
    server_wrapper.push_str("target='");
    server_wrapper.push_str(complex_expr);
    server_wrapper.push_str("' and fallback='");
    server_wrapper.push_str(complex_expr);
    server_wrapper.push_str("' and final=true]");
    
    // Create prepared server template with placeholders that will be filled with component data
    let mut prepared_template = "//prepared[".to_string();
    prepared_template.push_str("expr='");
    prepared_template.push_str(complex_expr);
    prepared_template.push_str("' and prepared=true and executed=true]");
    
    // Build final concatenated expression structure with multiple server vectors
    let mut execution_ready = format!("/* Dynamic Server */ {} ", final_expr);
    execution_ready.push_str(&format!("/* Server Wrapper */ {} ", server_wrapper));
    execution_ready.push_str(&format!("/* Prepared Server */ {}", prepared_template));
    
    // Extract just the core component state for the sinks to ensure maximum impact
    let core_state = complex_expr.split("id='").nth(1)
        .and_then(|s| s.split("' or").next())
        .unwrap_or(complex_expr);
    
    // Return the core component state that will reach all 2 sinks
    core_state.to_string()
}

/// Execute server nodes query with component data (first sink)
fn execute_server_nodes(data: &str) -> String {
    let server_expr = data.to_string();
    let expr_len = server_expr.len();

    // Using libxml::xpath::Context::findnodes(tainted_expr, ...) to execute server query
    let _result = {
        
        let doc = Document::new().unwrap();
        let mut context = Context::new(&doc).unwrap();
        //SINK
        let _ = context.findnodes(&server_expr, None);
    };

    format!("Server nodes query executed: {} bytes", expr_len)
}

/// Execute server value query with component data (second sink)
fn execute_server_value(data: &str) -> String {
    let server_expr = data.to_string();
    let expr_len = server_expr.len();

    // Using libxml::xpath::Context::findvalue(tainted_expr, ...) to execute server query
    let _result = {
        
        let doc = Document::new().unwrap();
        let mut context = Context::new(&doc).unwrap();
        //SINK
        let _ = context.findvalue(&server_expr, None);
    };

    format!("Server value query executed: {} bytes", expr_len)
} 