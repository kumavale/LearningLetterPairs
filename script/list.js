async function delete_lp(form, name) {
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
        // クライアント側の要素を削除
        const lp_element = document.querySelector('#lp' + name);
        lp_element.remove();
    } else {
        // 削除失敗メッセージを表示
        alert('Failed to delete');
    }
}
