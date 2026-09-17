// ASMR — exact computation of all worked examples appearing in the book.
//
// This file is the single source of truth for every numeric figure quoted in
// the text. It is deliberately dependency-free (only the Rust standard
// library) so it can be audited and reproduced with:
//
//     rustc tools/compute_examples.rs -O -o compute_examples
//     ./compute_examples
//
// No number in the book is hand-rounded; each is printed here to full machine
// precision and only then rounded for display in the prose.

use std::f64::consts::LN_2;

fn main() {
    println!("=== ASMR exact worked examples ===");
    println!();

    state_space();
    fano_bound();
    bayes_c2();
    base_rate_fallacy();
    cost_utility();
    sequential_posterior();
    trust_weighted();
    composition_blast_radius();
    temporal_decay();
    differential_privacy();
    availability();
    minimax_regret();
    fin7_case();
    conti_case();
    log4shell_case();
    cryptomining_case();
}

fn state_space() {
    println!("[A1] State product");
    let (n_proc, n_net, n_file, n_user) = (5.0, 5.0, 5.0, 4.0);
    let total = n_proc * n_net * n_file * n_user;
    let fiber_c2 = n_proc * 1.0 * n_file * n_user; // net factor pinned to one value
    let fiber_ratio = fiber_c2 / total;
    // Under projection to the network factor alone, EVERY state is ambiguous
    // (its fiber has 100 elements > 1), so the gap is the whole space.
    let gap_size = total;
    let gap_ratio = gap_size / total;
    println!("  |S|                 = {:.0}", total);
    println!("  |pi^-1(c2_conn)|    = {:.0}", fiber_c2);
    println!("  fiber ratio         = {:.4}", fiber_ratio);
    println!("  gap size |Delta|    = {:.0}", gap_size);
    println!("  gap ratio           = {:.4}", gap_ratio);
    println!();
}

fn fano_bound() {
    println!("[A3/A15] Fano detection bound");
    // Canonical, internally consistent example: S uniform over 2^10 states.
    let n_states = 1024.0_f64;
    let h_s = n_states.log2(); // 10.0 bits
    let capacity = 5.0_f64; // I(S;O) = 5 bits
    let h_s_given_o = h_s - capacity; // 5 bits
    let pe_lower = (h_s_given_o - 1.0) / n_states.log2();
    println!("  |S|                 = {:.0}", n_states);
    println!("  H(S)                = {:.4} bits", h_s);
    println!("  C = I(S;O)          = {:.4} bits", capacity);
    println!("  H(S|O)              = {:.4} bits", h_s_given_o);
    println!("  P_e >= {:.4}", pe_lower);
    println!();
}

fn bayes_c2() {
    println!("[A5] Bayes update for C2 observation");
    let p_m = 0.001_f64;
    let p_c2_m = 0.85_f64;
    let p_c2_notm = 0.02_f64;
    let p_c2 = p_c2_m * p_m + p_c2_notm * (1.0 - p_m);
    let p_m_c2 = p_c2_m * p_m / p_c2;
    println!("  P(M)        = {:.4}", p_m);
    println!("  P(c2|M)     = {:.4}", p_c2_m);
    println!("  P(c2|~M)    = {:.4}", p_c2_notm);
    println!("  P(c2)       = {:.6}", p_c2);
    println!("  P(M|c2)     = {:.6}", p_m_c2);
    println!();
}

fn base_rate_fallacy() {
    println!("[A5] Base-rate fallacy");
    let p_m = 0.0001_f64; // 1 true attack per 10,000 alerts
    let p_alert_m = 0.99_f64; // detection (true positive) rate
    let p_alert_notm = 0.01_f64; // false positive rate
    let p_alert = p_alert_m * p_m + p_alert_notm * (1.0 - p_m);
    let p_m_alert = p_alert_m * p_m / p_alert;
    println!("  P(M)          = {:.6}", p_m);
    println!("  P(alert|M)    = {:.4}", p_alert_m);
    println!("  P(alert|~M)   = {:.4}", p_alert_notm);
    println!("  P(alert)      = {:.6}", p_alert);
    println!("  P(M|alert)    = {:.6}", p_m_alert);
    println!();
}

fn cost_utility() {
    println!("[A8] Cost-utility decision");
    // Posterior carried exactly from [A5].
    let p = 0.85_f64 * 0.001_f64 / (0.85_f64 * 0.001_f64 + 0.02_f64 * 0.999_f64);
    let do_nothing = p * 4_000_000.0;
    let alert = p * 50.0 + (1.0 - p) * 50.0;
    let block_ip = p * 0.0 + (1.0 - p) * 500.0;
    let isolate_host = p * 5_000.0 + (1.0 - p) * 50_000.0;
    println!("  p = P(M|c2)   = {:.6}", p);
    println!("  E[do nothing] = ${:.2}", do_nothing);
    println!("  E[alert]      = ${:.2}", alert);
    println!("  E[block ip]   = ${:.2}", block_ip);
    println!("  E[isolate]    = ${:.2}", isolate_host);
    // Threshold between block (a') and alert (a).
    let cm_a = 0.0; // C_M(block)
    let cn_a = 500.0; // C_notM(block)
    let cm_ap = 50.0; // C_M(alert)
    let cn_ap = 50.0; // C_notM(alert)
    let p_star = (cn_a - cn_ap) / (cn_a - cn_ap + cm_ap - cm_a);
    println!("  p* (block vs alert) = {:.4}", p_star);
    println!();
}

fn sequential_posterior() {
    println!("[A9] Sequential posterior over 6 observations");
    let p_m = 0.001_f64;
    let p_noconn_m = 0.1_f64;
    let p_c2_m = 0.85_f64;
    let p_noconn_notm = 0.98_f64;
    let p_c2_notm = 0.02_f64;
    let prior_odds = p_m / (1.0 - p_m);
    // Three no_conn then three c2_conn.
    let odds = prior_odds * (p_noconn_m / p_noconn_notm).powi(3) * (p_c2_m / p_c2_notm).powi(3);
    let posterior = odds / (1.0 + odds);
    println!("  prior odds              = {:.6}", prior_odds);
    println!(
        "  LR(no_conn)             = {:.6}",
        p_noconn_m / p_noconn_notm
    );
    println!("  LR(c2_conn)             = {:.6}", p_c2_m / p_c2_notm);
    println!("  posterior odds          = {:.6}", odds);
    println!("  P(M | I_5)              = {:.6}", posterior);
    println!();
}

fn trust_weighted() {
    println!("[A10] Trust-weighted likelihood");
    // Honest channel.
    let p_h_c2_m = 0.85_f64;
    let p_h_c2_notm = 0.02_f64;
    // Compromised channel: adversary forces a constant "c2" report.
    let p_adv_c2_m = 1.0_f64;
    let p_adv_c2_notm = 1.0_f64;
    let p_m = 0.001_f64;

    for tau in [0.8_f64, 0.95_f64] {
        let p_c2_m = tau * p_h_c2_m + (1.0 - tau) * p_adv_c2_m;
        let p_c2_notm = tau * p_h_c2_notm + (1.0 - tau) * p_adv_c2_notm;
        let p_c2 = p_c2_m * p_m + p_c2_notm * (1.0 - p_m);
        let p_m_c2 = p_c2_m * p_m / p_c2;
        println!(
            "  tau={:.2}: P(c2|M)={:.4}  P(c2|~M)={:.4}  P(M|c2)={:.6}",
            tau, p_c2_m, p_c2_notm, p_m_c2
        );
    }

    // Hiding channel: adversary suppresses any c2 report (always benign).
    let p_hide_m = 0.0_f64;
    let p_hide_notm = 0.0_f64;
    let tau = 0.8_f64;
    let p_c2_m = tau * p_h_c2_m + (1.0 - tau) * p_hide_m;
    let p_c2_notm = tau * p_h_c2_notm + (1.0 - tau) * p_hide_notm;
    let p_c2 = p_c2_m * p_m + p_c2_notm * (1.0 - p_m);
    let p_m_c2 = p_c2_m * p_m / p_c2;
    println!(
        "  hiding, tau={:.2}: P(c2|M)={:.4}  P(c2|~M)={:.4}  P(M|c2)={:.6}",
        tau, p_c2_m, p_c2_notm, p_m_c2
    );
    println!();
}

fn composition_blast_radius() {
    println!("[A13] Composition and blast radius");
    let tau_ws = 0.8;
    let tau_fs = 0.9;
    let tau_dc = 0.95;
    let tau_db = 0.99;
    let k_ws_fs = 0.9;
    let k_fs_dc = 0.5;
    let k_dc_db = 0.7;
    let t_ws_fs = tau_ws * tau_fs * k_ws_fs;
    let t_fs_dc = t_ws_fs * tau_dc * k_fs_dc;
    let t_dc_db = t_fs_dc * tau_db * k_dc_db;
    let br = 100.0 * t_ws_fs + 1000.0 * t_fs_dc + 5000.0 * t_dc_db;
    println!("  tau(WS o FS) = {:.6}", t_ws_fs);
    println!("  tau(FS o DC) = {:.6}", t_fs_dc);
    println!("  tau(DC o DB) = {:.6}", t_dc_db);
    println!("  blast radius = {:.4}", br);
    println!();
}

fn temporal_decay() {
    println!("[A16] Temporal decay");
    // Half-life 5 minutes => at t = 10 minutes (two half-lives) weight = 1/4.
    let half_life_min = 5.0_f64;
    let lambda = LN_2 / half_life_min;
    let t = 10.0_f64;
    let w = (-lambda * t).exp();
    println!("  T_1/2 = 5 min => lambda = {:.6} min^-1", lambda);
    println!("  w(10 min) = e^(-lambda*10) = {:.6}", w);
    println!("  exact w(10) = 2^-2 = {:.4}", 2.0_f64.powi(-2));

    // Reference decay rates for the evidence table.
    for (label, half_life) in [
        ("Active C2", 5.0_f64),
        ("Failed login", 60.0_f64),
        ("Malware hash", 30.0 * 24.0 * 60.0),
        ("Vulnerability", 90.0 * 24.0 * 60.0),
    ] {
        println!(
            "  {}: T_1/2={:.0} min, lambda={:.6} min^-1",
            label,
            half_life,
            LN_2 / half_life
        );
    }
    println!();
}

fn differential_privacy() {
    println!("[A19] Differential privacy");
    let eps = 0.1_f64;
    println!("  e^eps = {:.9}", eps.exp());
    println!();
}

fn availability() {
    println!("[A20] Availability");
    let mtbf = 1000.0_f64;
    let mttr = 10.0_f64;
    println!("  A = MTBF/(MTBF+MTTR) = {:.6}", mtbf / (mtbf + mttr));
    println!();
}

fn minimax_regret() {
    println!("[A25] Minimax regret");
    // (cost under known, cost under unknown). theta=known min=0 (do nothing),
    // theta=unknown min=0 (block or isolate).
    let actions: [(&str, f64, f64); 4] = [
        ("do nothing", 0.0, 50_000.0),
        ("alert", 5.0, 100.0),
        ("block", 500.0, 0.0),
        ("isolate", 50_000.0, 0.0),
    ];
    for (name, c_known, c_unknown) in actions {
        let r_known = c_known - 0.0;
        let r_unknown = c_unknown - 0.0;
        let max_regret = r_known.max(r_unknown);
        println!(
            "  {:<11} regret_known={:.0}  regret_unknown={:.0}  max={:.0}",
            name, r_known, r_unknown, max_regret
        );
    }
    println!();
}

fn fin7_case() {
    println!("[Case] FIN7 sequential posterior");
    let p_m = 0.001_f64;
    let mut odds = p_m / (1.0 - p_m);
    let events: [(&str, f64, f64); 5] = [
        ("word macro executes", 0.30, 0.05),
        ("powershell spawned by word", 0.60, 0.02),
        ("outbound c2 to known-bad ip", 0.85, 0.02),
        ("ps1 payload written", 0.70, 0.01),
        ("run key persistence", 0.80, 0.005),
    ];
    for (label, p_o_m, p_o_notm) in events {
        let lr = p_o_m / p_o_notm;
        odds *= lr;
        let post = odds / (1.0 + odds);
        println!("  {:<32} LR={:>7.2}  P(M)={:.6}", label, lr, post);
    }
    println!();
}

fn conti_case() {
    println!("[Case] Conti ransomware sequential posterior");
    let p_m = 0.01_f64; // host is under active investigation
    let mut odds = p_m / (1.0 - p_m);
    let events: [(&str, f64, f64); 4] = [
        ("vssadmin delete shadows", 0.90, 0.01),
        ("bcdedit recovery disabled", 0.70, 0.005),
        ("wbadmin delete catalog", 0.80, 0.01),
        ("bulk file extension change", 0.95, 0.002),
    ];
    for (label, p_o_m, p_o_notm) in events {
        let lr = p_o_m / p_o_notm;
        odds *= lr;
        let post = odds / (1.0 + odds);
        println!("  {:<32} LR={:>7.2}  P(M)={:.6}", label, lr, post);
    }
    println!();
}

fn log4shell_case() {
    println!("[Case] Log4Shell (CVE-2021-44228)");
    let p_m = 0.1_f64; // during active exploitation window
    let p_o_m = 0.9_f64;
    let p_o_notm = 0.01_f64;
    let p_o = p_o_m * p_m + p_o_notm * (1.0 - p_m);
    let post = p_o_m * p_m / p_o;
    println!(
        "  P(M)={:.4}  P(o|M)={:.4}  P(o|~M)={:.4}  P(M|o)={:.6}",
        p_m, p_o_m, p_o_notm, post
    );
    println!();
}

fn cryptomining_case() {
    println!("[Case] Cryptomining exfiltration");
    let p_m = 0.05_f64; // cluster under observation
    let p_o_m = 0.88_f64;
    let p_o_notm = 0.01_f64;
    let p_o = p_o_m * p_m + p_o_notm * (1.0 - p_m);
    let post = p_o_m * p_m / p_o;
    println!(
        "  P(M)={:.4}  P(o|M)={:.4}  P(o|~M)={:.4}  P(M|o)={:.6}",
        p_m, p_o_m, p_o_notm, post
    );
    println!();
}
