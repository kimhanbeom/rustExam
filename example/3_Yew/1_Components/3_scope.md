1.Scope
-----------
- Component의 scope는 1)Component가 callback을 생성하고  2)메세지를 통해 자체적으로 update 할 수 있는 메커니즘입니다.

- Component에 전달된 context object에서 link()를 호출함으로써 scope의 reference를 얻을 수 있습니다.


2.send_message
--------------
- Component에 메세지를 전달합니다.
- 메세지는 update 메서드에서 handle합니다.(component의 re-render 관련하여 결정여부)

3.send_message_batch
--------------------
- Component에 동시에 다중 메세지를 보냅니다.
- send_message와 유사하지만, 만약 특정 메세지가 update 메서드에서 true를 return 시킨다면(component가 re-render해야된다는 뜻), component는 batch의 모든 메세제를 처리한 후 re-render 할 것입니다.


4.callback
----------
- 실행 될때 Component에 메세지를 보낼 수 있는 callback을 만듭니다.
- callback 내부에서는 closuere에서 반환한 메세지와 함께 send_message를 호출합니다.
- 한번만 실행하기 위한 callback_once 메서드도 있습니다.(그이상 호출시 panic발생하니 주의)

```rust
    use yew::{html, Component, Context, Html};

    enum Msg {
        Text(String),
    }

    struct Comp;

    impl Component for Comp {

        type Message = Msg;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            // Create a callback that accepts some text and sends it
            // to the component as the `Msg::Text` message variant.
            let cb = ctx.link().callback(|text: String| Msg::Text(text));

            // The previous line is needlessly verbose to make it clearer.
            // It can be simplified it to this:
            let cb = ctx.link().callback(Msg::Text);

            // Will send `Msg::Text("Hello World!")` to the component.
            cb.emit("Hello World!".to_owned());

            html! {
                // html here
            }
        }
    }
```


5.batch_callback
----------------
- 실행 될 때 Component에 batch 메세지를 보낼 수 있는 callback을 만듭니다.
- callback과 차이점은 이 메서드에 전달된 closure은 메세지를 리턴할 수 없습니다.
- 대신에 closuer는 Vec<Msg>나 Option<Msg>를 리턴 할 수 있습니다.
- Vec<Msg>는 batch의 message로써 사용되고, send_message_batch 내부에서 사용됩니다.
- Option<Msg>은 Some인경우 send_message를 호출하고 None인 경우 아무것도 하지 않습니다. 업데이트가 필요하지 않은 상황에서 사용되곤 합니다.
- 유저가 만든 자신의 타입에 SendAsMessage trait을 구현하여 batch_callback에서 사용할 수 있습니다.
- 한번만 실행하기 위한 batch_callback_once 메서드도 있습니다.(그이상 호출시 panic발생하니 주의)
