extern "C"
{
#include "node_api.h"
}

#include <iostream>
#include <vector>
#include <random>
#include <ctime>

bool to_exit_process;
int message_count = 0; // 消息计数器
const int max_messages = 10000; // 最大消息数

std::string generate_random_string(size_t length)
{
    const char charset[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    std::default_random_engine engine(static_cast<unsigned int>(time(0)));
    std::uniform_int_distribution<> dist(0, sizeof(charset) - 2);
    std::string random_string;

    for (size_t i = 0; i < length; ++i)
    {
        random_string += charset[dist(engine)];
    }

    return random_string;
}

int run(void *dora_context)
{
    to_exit_process = false;

    std::default_random_engine engine(static_cast<unsigned int>(time(0)));
    std::uniform_int_distribution<> length_dist(5, 20); // 随机字符串长度范围 5 到 20

    while (!to_exit_process && message_count < max_messages)
    {
        void *event = dora_next_event(dora_context);
        if (event == NULL)
        {
            printf("[c node] ERROR: unexpected end of event\n");
            return -1;
        }

        enum DoraEventType ty = read_dora_event_type(event);

        if (ty == DoraEventType_Input)
        {
            size_t random_length = length_dist(engine); // 生成随机长度
            std::string random_string = generate_random_string(random_length);
            char* output_data = const_cast<char*>(random_string.c_str());

            std::string out_id = "counter_A";
            size_t data_len = random_string.size();
            int resultend = dora_send_output(dora_context, &out_id[0], out_id.length(), output_data, data_len);

            message_count++; // 递增消息计数器

            std::cout << "dora_send_output: out_id " << out_id << "  out_data_len: " << data_len << std::endl;
            std::cout << "Sent random string: " << random_string << std::endl; // 打印发送的随机字符串
            std::cout << "Message count: " << message_count << std::endl; // 打印消息计数器

            if (resultend != 0)
            {
                std::cerr << "failed to send output" << std::endl;
                return 1;
            }
        }
        else if (ty == DoraEventType_Stop)
        {
            printf("[c node] received stop event\n");
        }
        else
        {
            printf("[c node] received unexpected event: %d\n", ty);
        }

        free_dora_event(event);
    }

    if (message_count >= max_messages)
    {
        std::cout << "Reached the maximum number of messages: " << max_messages << std::endl;
        to_exit_process = true;
    }

    return 0;
}

int main()
{
    std::cout << "dora node A driver" << std::endl;

    auto dora_context = init_dora_context_from_env();
    auto ret = run(dora_context);
    free_dora_context(dora_context);

    to_exit_process = true;

    std::cout << "exit dora node A ..." << std::endl;
    return ret;
}
