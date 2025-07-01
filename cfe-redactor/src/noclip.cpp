#include "fe-settings.h"
#include "fe-includes.h"
#include "FBO.h"
#include "camera.h"
#include "model.h"
#include "shader.h"
#include "window.h"
#include "main.h"
#include "textRenderer.h"
#include "fe-logic.h"

/* ImGui */
#include "imgui.h"
#include "imgui_stdlib.h"
#include "imgui_impl_glfw.h"
#include "imgui_impl_opengl3.h"
#include <GLFW/glfw3.h>
#include <bits/c++config.h>
#include <cmath>
#include <ostream>
#include <cstring>
#include "console.h"

extern Camera * mainCamera;
extern Shader * modelDrawShader;
std::string inputbufstr;
Console * cnsl = 0;

FeTestApp::FeTestApp(void)
    : input(fe_getInput()), 
      shader(fe_getShader(0)), 
      outlineShader(fe_getShader(1)), 
      txtShader(new Shader("shaders/shader_txt.glsl")),
      txtRenderer(new TextRenderer(*txtShader, "assets/fonts/ProggyCleanRu.ttf", 40)),
      window(fe_getWindow())
{
    cnsl = new Console();
		//cnsl_in = std::thread(cnsl->process_input);

    camera = new Camera(WINDOW_WIDTH, WINDOW_HEIGHT, glm::vec3(-2.f, 8.f, 4.f), 45.0f, 0.1f, 100.0f);
    mainCamera = camera;
    Model * model  = new Model("assets/models/vec3arr/scene.gltf");
    Model * model1 = new Model("assets/models/sword/scene.gltf");
    model->shType = 0;
    model->enablePicking = 0;

    model->meshes[0].enablePicking = 1;

    std::cout << "sword: " << model1->getId() << std::endl;
    std::cout << "vec3arr: " << model->getId() << std::endl;
    std::cout << "vec3arr meshes: " << model->meshes.size() << std::endl;
    glfwSetWindowUserPointer(window->getWindow(), camera);
    setupImGui(window->getWindow());

    addModel(model);
    addModel(model1);

    ImGuiStyle &style = ImGui::GetStyle();
    style.WindowPadding = ImVec2(4, 4);
    style.ItemSpacing = ImVec2(8, 4);
    style.ItemInnerSpacing = ImVec2(5, 4);
    style.ScrollbarSize = 15;
    style.GrabMinSize = 15;
    style.WindowBorderSize = 0;
    style.ChildBorderSize = 0;
    style.TabBarBorderSize = 0;
    style.WindowRounding = 5;
    style.FrameRounding = 6;
    style.FramePadding = ImVec2(6, 6);
    style.ScrollbarRounding = 12;
    style.GrabRounding = 4;
    style.TableAngledHeadersTextAlign = ImVec2(0.5, 0.5);
    style.WindowMenuButtonPosition = ImGuiDir_Right;
    style.ColorButtonPosition = ImGuiDir_Left;
    style.SeparatorTextBorderSize = 1;
    style.SeparatorTextAlign = ImVec2(0.5, 0.5);
    style.WindowTitleAlign = ImVec2(0.5, 0.5);
    style.SeparatorTextPadding = ImVec2(0.0, 0.0);

    style.Colors[ImGuiCol_TitleBgActive] = ImColor{IM_COL32(112, 19, 208, 255)};
    style.Colors[ImGuiCol_TitleBg] = ImColor{IM_COL32(23, 11, 36, 255)};
    style.Colors[ImGuiCol_WindowBg] = ImColor{IM_COL32(0, 0, 0, 255)};
    style.Colors[ImGuiCol_FrameBgHovered] = ImColor{IM_COL32(177, 98, 255, 100)};
    style.Colors[ImGuiCol_FrameBg] = ImColor{IM_COL32(81, 41, 122, 95)};
    style.Colors[ImGuiCol_CheckMark] = ImColor{IM_COL32(125, 0, 255, 255)};
    style.Colors[ImGuiCol_SliderGrab] = ImColor{IM_COL32(151, 0, 255, 255)};
    style.Colors[ImGuiCol_Button] = ImColor{IM_COL32(106, 46, 255, 130)};
    style.Colors[ImGuiCol_Header] = ImColor{IM_COL32(88, 28, 170, 150)};
    style.Colors[ImGuiCol_Tab] = ImColor{IM_COL32(92, 66, 167, 220)};
    style.Colors[ImGuiCol_TabSelected] = ImColor{IM_COL32(182, 128, 255, 255)};
    style.Colors[ImGuiCol_TabHovered] = ImColor{IM_COL32(177, 150, 255, 255)};

    ImGuiIO &io = ImGui::GetIO(); (void)io;

    //io.Fonts->AddFontDefault();
    ImFont *mainft = io.Fonts->AddFontFromFileTTF("assets/fonts/JetBrainsMonoNerdFontPropo-Medium.ttf", 21.f, 0, io.Fonts->GetGlyphRangesCyrillic());
    //IM_ASSERT(!mainft);
}

void FeTestApp::cycle(void)
{
    if (window->windowGetMouseButton(GLFW_MOUSE_BUTTON_RIGHT, GLFW_PRESS)) {
        FBO * fbo = fe_getFBO();
        double mouseX, mouseY;
        glfwGetCursorPos(window->getWindow(), &mouseX, &mouseY);
        uint modelID = fbo->getModelID((int)mouseX, (int)mouseY);

        std::cout << "ID in big-endian: " << modelID << std::endl;
    }

    if (window->windowGetKey(GLFW_KEY_F, GLFW_PRESS)) {
        Model * model = getModel(0);
        model->pos.x += 0.1f;
    }

    if (window->windowGetKey(GLFW_KEY_G, GLFW_PRESS)) {
        Model * model = getModel(0);
        model->pos.x -= 0.1f;
    }


    // TODO: add drawOutline func
    /*glStencilFunc(GL_ALWAYS, 1, 0xff);
    glStencilMask(0xff);*/
/*

    glStencilFunc(GL_NOTEQUAL, 1, 0xff);
    glStencilMask(0x00);
    glDisable(GL_DEPTH_TEST);
    outlineShader->bind();
    outlineShader->setUniform("outlining", 0.08f);
    model->draw(*outlineShader, *camera);

    glStencilMask(0xff);
    glStencilFunc(GL_ALWAYS, 0, 0xff);
    glEnable(GL_DEPTH_TEST);*/

    if (camera->showImGui) {
        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();
        ImGuiStyle &style = ImGui::GetStyle();

        ImGui::ShowDemoWindow();

        ImGui::Begin("Settings");
            ImGui::SliderFloat("Speed", &camera->speed, 0.001, 0.5);
        ImGui::End();

        ImGui::Begin("Console");
            ImVec4 origFrameBg = style.Colors[ImGuiCol_FrameBg];
            ImVec4 origFrameBgActive = style.Colors[ImGuiCol_FrameBgActive];

            style.Colors[ImGuiCol_FrameBg] = ImVec4(0, 0, 0, 0);
            style.Colors[ImGuiCol_FrameBgActive] = ImVec4(0, 0, 0, 0);

            const float footer_height = ImGui::GetStyle().ItemSpacing.y + ImGui::GetFrameHeightWithSpacing();
            ImGui::BeginChild("ScrollingRegion", ImVec2(0, -footer_height), ImGuiChildFlags_NavFlattened, ImGuiWindowFlags_HorizontalScrollbar);
                std::string output = cnsl->oss.str();
                ImGui::InputTextMultiline("##termtxt", const_cast<char *>(output.c_str()), output.size(), ImVec2(-1, -1), ImGuiInputTextFlags_ReadOnly);
            ImGui::EndChild();

            ImGui::Separator();
            ImGui::SetNextItemWidth(-1);

            ImGui::InputTextWithHint("##input", "enter command", &inputbufstr);

						if (!inputbufstr.empty() && ImGui::IsItemHovered() && window->windowGetKey(GLFW_KEY_ENTER, GLFW_PRESS)) {
							cnsl->input(inputbufstr);
							inputbufstr.clear();
						}
						
            style.Colors[ImGuiCol_FrameBg] = origFrameBg;
            style.Colors[ImGuiCol_FrameBgActive] = origFrameBgActive;
        ImGui::End();

        ImGui::Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
    }

    txtRenderer->RenderText(*txtShader, "Sample text", 25.0f, 25.0f, .5f, glm::vec3(0.5, 0.8f, 0.2f));
}

void FeTestApp::input_callback(Window * window, Camera &cam)
{
    if (!cam.showImGui) {
        if (window->windowGetKey(GLFW_KEY_W, GLFW_PRESS)) {
            cam.pos += cam.speed * cam.orientation;
        }
        if (window->windowGetKey(GLFW_KEY_A, GLFW_PRESS)) {
            cam.pos -= cam.speed * glm::normalize(glm::cross(cam.orientation, cam.up));
        }
        if (window->windowGetKey(GLFW_KEY_S, GLFW_PRESS)) {
            cam.pos -= cam.speed * cam.orientation;
        }
        if (window->windowGetKey(GLFW_KEY_D, GLFW_PRESS)) {
            cam.pos += cam.speed * glm::normalize(glm::cross(cam.orientation, cam.up));
        }
        if (window->windowGetKey(GLFW_KEY_SPACE, GLFW_PRESS)) {
            cam.pos += cam.speed * cam.up;
        }
        if (window->windowGetKey(GLFW_KEY_LEFT_CONTROL, GLFW_PRESS)) {
            cam.pos -= cam.speed * cam.up;
        }

        if (window->windowGetKey(GLFW_KEY_P, GLFW_PRESS)) {
            cam.speed = 0.4f;
        }
        if (window->windowGetKey(GLFW_KEY_P, GLFW_RELEASE)) {
            //cam.speed = 0.1f;
        }

        #ifdef LOG_LEVEL_INFO
        std::cout << cam.pos.x << std::endl;
        std::cout << cam.pos.y << std::endl;
        std::cout << cam.pos.z << std::endl;
        #endif

        if (window->windowGetMouseButton(GLFW_MOUSE_BUTTON_LEFT, GLFW_PRESS) && !cam.showImGui) {
            window->hideCursor();

            glm::vec2 center = {
                cam.w/2,
                cam.h/2,
            };

            if (firstClick) {
                window->setCursorPos(center.x, center.y);
                firstClick = false;
            }

            double mouseX, mouseY;
            window->getCursorPos(&mouseX, &mouseY);

            float deltaX = (float)(mouseX - center.x);
            float deltaY = (float)(mouseY - center.y);

            float rotX = cam.sensitivity * deltaY / cam.h;
            float rotY = cam.sensitivity * deltaX / cam.w;
            glm::vec3 newOrientation = glm::rotate(cam.orientation, glm::radians(-rotX), glm::normalize(glm::cross(cam.orientation, cam.up)));

            if (abs((int)glm::angle(newOrientation, cam.up) - (int)glm::radians(90.0f)) <= glm::radians(85.0f)) {
                cam.orientation = newOrientation;
            }

            cam.orientation = glm::rotate(cam.orientation, glm::radians(-rotY), cam.up);

            #ifdef FE_LOG_LEVEL_INFO
            std::cout << cam.orientation.x << std::endl;
            std::cout << cam.orientation.y << std::endl;
            std::cout << cam.orientation.z << std::endl;
            #endif
            window->setCursorPos(center.x, center.y);
        }

        else if (window->windowGetMouseButton(GLFW_MOUSE_BUTTON_LEFT, GLFW_RELEASE)) {
            window->normalCursor();
            firstClick = true;
        }
    }
}

void FeTestApp::free(void)
{
    delete txtShader;
    delete txtRenderer;
    delete camera;
    delete cnsl;
}
