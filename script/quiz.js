// レターペアの循環リスト
// TODO: リストにする
var LP_LISTS;
var LP_POS = 0;  // 現在LP_LISTSの何番目を表示しているか

window.onload = function() {
    // シャッフルされたレターペアのリストを取得
    fetch('/shuffle_lp', { method: 'POST' })
        .then((response) => {
            return response.json();
        })
        .then((jsonobj) => {
            // グローバル変数に格納
            LP_LISTS = jsonobj.lists;
            // 問題を表示
            display_question();
        })
        .catch((e) => {
            alert("Failed to get contents");
        });

    // [Answer]ボタン押下時のイベントハンドラ設定
    var control_answer = document.querySelector('#control_answer');
    control_answer.addEventListener('click', () => { show_answer('visible'); });

    // [Next]ボタン押下時のイベントハンドラ設定
    var control_next = document.querySelector('#control_next');
    control_next.addEventListener('click', next_lp);

    // [Prev]ボタン押下時のイベントハンドラ設定
    var control_prev = document.querySelector('#control_prev');
    control_prev.addEventListener('click', prev_lp);
};

// ショートカットキーの設定
document.addEventListener('keydown', (e) => {
    switch (e.key) {
        case 'ArrowLeft':
            prev_lp();
            break;
        case 'ArrowRight':
            next_lp();
            break;
        case ' ':
            e.preventDefault();
            show_answer('visible');
            break;
        default:
            break;  // Do nothing
    }
});

// 問題を表示
function display_question() {
    // 解答を非表示に設定
    show_answer('hidden');

    // LPメニューの設定
    var modify_lp_element = document.querySelector('#modify_input_lp');
    modify_lp_element.value = LP_LISTS[LP_POS].name;
    var delete_lp_element = document.querySelector('#delete_input_lp');
    delete_lp_element.value = LP_LISTS[LP_POS].name;

    // レターペアのタイトルを設定
    var lp_name_element = document.querySelector('#lp_name');
    lp_name_element.innerText = LP_LISTS[LP_POS].name;

    // レターペアの内容を設定
    var lp_objects_element = document.querySelector('#lp_objects');
    lp_objects_element.innerHTML = '';
    LP_LISTS[LP_POS].objects.forEach(object => {
        var li = document.createElement('li');
        li.innerText = object;
        lp_objects_element.appendChild(li);
    });

    // 画像ファイルの設定
    var lp_image_element = document.querySelector('#lp_image');
    lp_image_element.innerHTML = '';
    var img = document.createElement('img');
    img.src = "static/img/" + LP_LISTS[LP_POS].image;
    lp_image_element.appendChild(img);
}

// visibility:hiddenになっている要素を可視化する
function show_answer(visibility) {
    var lp_objects_element = document.querySelector('#lp_objects');
    var lp_image_element   = document.querySelector('#lp_image');
    lp_objects_element.style.visibility = visibility;
    lp_image_element  .style.visibility = visibility;
}

// 次の問題を表示
function next_lp() {
    if (LP_POS < LP_LISTS.length - 1) {
        LP_POS += 1;
    } else {
        LP_POS = 0;
    }
    display_question();
}

// 前の問題を表示
function prev_lp() {
    if (LP_POS > 0) {
        LP_POS -= 1;
    } else {
        LP_POS = LP_LISTS.length - 1;
    }
    display_question();
}

// LPの削除処理
async function delete_lp(form) {
    let name = LP_LISTS[LP_POS].name;
    if (!confirm('Delete ' + name + '?')) {
        // 削除キャンセル
        return;
    }

    // 削除処理をPOST
    const action = form.getAttribute("action");
    const options = {
        method: 'POST',
        body: new URLSearchParams(new FormData(form)),
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
    };
    const res = await fetch(action, options);

    if (res.ok) {
        // LPメニューを非表示にする
        document.querySelector('details').removeAttribute("open");
        // 現在表示しているLPを削除して次の問題を表示する
        LP_LISTS.splice(LP_POS, 1);
        if (LP_POS >= LP_LISTS.length) {
            LP_POS = 0;
        }
        display_question();
    } else {
        // 削除失敗メッセージを表示
        alert('Failed to delete');
    }
}

// LPを検索して表示する
function search() {
    const current_pos = LP_POS;
    const end_pos = (() => { if (LP_POS > 0) { return LP_POS - 1; } else { return LP_LISTS.length - 1; } })();
    let search_lp = document.getElementById("search_lp").value;
    while (LP_POS != end_pos) {
        if (LP_LISTS[LP_POS].name == search_lp) {
            // 表示して関数を抜ける
            display_question();
            return;
        }
        // 次の問題へシフト
        if (LP_POS < LP_LISTS.length - 1) {
            LP_POS += 1;
        } else {
            LP_POS = 0;
        }
    }
    // 見つからなかったらLP_POSをリセット
    LP_POS = current_pos;
}
